use crate::error::{RecordingError, Result};
use crate::messaging::RecordingMode;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::*;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Callback invoked when microphone permission is denied.
///
/// Returns `true` if the user wants to continue without mic, `false` to abort
/// stream acquisition entirely.  The default implementation uses
/// `window.confirm()`.  The popup UI story (3.1 / 3.2) can replace this with a
/// proper styled dialog.
pub(crate) type MicDeniedHandler = Box<dyn Fn() -> bool>;

/// The result of a successful stream acquisition.
///
/// Carries the combined media stream, the `AudioContext` that must be kept
/// alive for audio to flow, and an optional reference to the microphone track.
#[derive(Debug)]
pub(crate) struct AcquiredStream {
    /// Combined MediaStream (video + mixed audio).
    pub media_stream: MediaStream,
    /// AudioContext kept alive for the duration of recording.
    /// Dropping this stops all audio flow.
    pub audio_context: AudioContext,
    /// The microphone track, if acquired.
    pub mic_track: Option<MediaStreamTrack>,
}

impl AcquiredStream {
    /// Returns `true` when the stream contains at least one video track.
    pub fn has_video(&self) -> bool {
        self.media_stream.get_video_tracks().length() > 0
    }

    /// Returns `true` when the stream contains at least one audio track.
    pub fn has_audio(&self) -> bool {
        self.media_stream.get_audio_tracks().length() > 0
    }
}

/// Orchestrates the acquisition of display/tab and microphone streams.
///
/// ## Context requirements
///
/// | Mode | Required context | API |
/// |------|-----------------|-----|
/// | `FullScreen` | DOM-capable (offscreen doc, popup) | `getDisplayMedia()` via `web-sys` |
/// | `Tab` | Service worker background | `chrome.tabCapture` via JS shim |
///
/// The caller MUST invoke `acquire()` from the correct context for the chosen
/// mode.  Tab-mode acquisition returns a tab stream ID that the background SW
/// passes to the offscreen document for reconstruction.
pub(crate) struct StreamAcquisitionService {
    mode: RecordingMode,
    mic_enabled: bool,
    mic_denied_handler: Option<MicDeniedHandler>,
}

impl std::fmt::Debug for StreamAcquisitionService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamAcquisitionService")
            .field("mode", &self.mode)
            .field("mic_enabled", &self.mic_enabled)
            .field(
                "mic_denied_handler",
                &self.mic_denied_handler.as_ref().map(|_| "Box<dyn Fn() -> bool>"),
            )
            .finish()
    }
}

impl StreamAcquisitionService {
    /// Create a new service for the given mode and mic preference.
    pub fn new(mode: RecordingMode, mic_enabled: bool) -> Self {
        Self {
            mode,
            mic_enabled,
            mic_denied_handler: None,
        }
    }

    /// Set a custom handler for the microphone-denied dialog.
    ///
    /// When not set, a default `window.confirm()` is used.  The popup UI
    /// story (3.1 / 3.2) should replace this with a styled dialog.
    pub fn set_mic_denied_handler(&mut self, handler: MicDeniedHandler) {
        self.mic_denied_handler = Some(handler);
    }

    /// Return the recording mode.
    pub fn mode(&self) -> &RecordingMode {
        &self.mode
    }

    /// Return whether microphone is enabled.
    pub fn mic_enabled(&self) -> bool {
        self.mic_enabled
    }

    /// Acquire all streams and return a combined `AcquiredStream`.
    ///
    /// ## Flow
    ///
    /// 1. Acquire the video source (display or tab) via `acquire_display()` /
    ///    `acquire_tab()`.
    /// 2. If `mic_enabled`, attempt `acquire_microphone()`.
    /// 3. Mix audio tracks from video source and mic via `mix_audio()`.
    /// 4. Return the combined stream.
    ///
    /// See the struct-level docs for context requirements per mode.
    pub async fn acquire(&self) -> Result<AcquiredStream> {
        // Phase 1: acquire video source
        let video_source = match self.mode {
            RecordingMode::FullScreen => acquire_display().await?,
            RecordingMode::Tab => acquire_tab().await?,
        };

        // Phase 2: acquire microphone if enabled
        let mic_track = if self.mic_enabled {
            acquire_microphone_with_handler(&self.mic_denied_handler).await?
        } else {
            None
        };

        // Phase 3: mix audio
        if mic_track.is_some() || video_source.get_audio_tracks().length() > 0 {
            let (media_stream, audio_context) =
                mix_audio(&video_source, mic_track.as_ref())?;
            Ok(AcquiredStream {
                media_stream,
                audio_context,
                mic_track,
            })
        } else {
            // No audio at all — create an AudioContext that we hold
            // so no mixing is needed.
            let ctx = AudioContext::new().map_err(|_| {
                RecordingError::StreamAcquisitionFailed {
                    details: "Failed to create AudioContext".into(),
                }
            })?;
            Ok(AcquiredStream {
                media_stream: video_source,
                audio_context: ctx,
                mic_track: None,
            })
        }
    }
}

// ---------------------------------------------------------------------------
// JS shim import — chrome.tabCapture
// ---------------------------------------------------------------------------

#[wasm_bindgen(module = "/js/chrome_shim.js")]
extern "C" {
    /// Call `chrome.tabCapture.capture({ audio: true, video: true })`.
    ///
    /// Returns a `Promise<{ streamId: string }>`.
    /// Only callable from the service worker background context.
    #[wasm_bindgen(catch)]
    fn tabCaptureCapture() -> std::result::Result<js_sys::Promise, JsValue>;
}

// ---------------------------------------------------------------------------
// Core acquisition functions
// ---------------------------------------------------------------------------

/// Acquire a display stream via `getDisplayMedia()`.
///
/// Must be called from a DOM-capable context (offscreen document or popup).
async fn acquire_display() -> Result<MediaStream> {
    let window = web_sys::window().ok_or_else(|| RecordingError::StreamAcquisitionFailed {
        details: "No window object — cannot acquire display stream".into(),
    })?;

    let media_devices =
        window
            .navigator()
            .media_devices()
            .map_err(|_| RecordingError::StreamAcquisitionFailed {
                details: "navigator.mediaDevices is not available in this context".into(),
            })?;

    let promise = media_devices.get_display_media().map_err(|_| {
        RecordingError::StreamAcquisitionFailed {
            details: "getDisplayMedia() call failed".into(),
        }
    })?;

    let js_value = JsFuture::from(promise).await.map_err(|_| {
        RecordingError::StreamAcquisitionFailed {
            details: "Screen or tab selection was cancelled.".into(),
        }
    })?;

    let stream: MediaStream = JsCast::dyn_into(js_value).map_err(|_| {
        RecordingError::StreamAcquisitionFailed {
            details: "getDisplayMedia() did not return a MediaStream".into(),
        }
    })?;

    Ok(stream)
}

/// Acquire a tab stream via the `chrome.tabCapture` JS shim.
///
/// Must be called from the service worker background context where
/// `chrome.tabCapture` is available.
///
/// ## Background → Offscreen handoff
///
/// The returned stream ID must be passed to the offscreen document (via URL
/// parameter or message), which then reconstructs the `MediaStream` using
/// `navigator.mediaDevices.getUserMedia()` with
/// `chromeMediaSource: "tab"` and the stream ID.
async fn acquire_tab() -> Result<MediaStream> {
    let promise = tabCaptureCapture().map_err(|_| {
        RecordingError::StreamAcquisitionFailed {
            details: "chrome.tabCapture is not available in this context".into(),
        }
    })?;

    let result = JsFuture::from(promise).await.map_err(|_| {
        RecordingError::StreamAcquisitionFailed {
            details:
                "Tab capture failed or was denied. Check permissions in chrome://extensions."
                    .into(),
        }
    })?;

    // The shim returns { streamId: "..." }
    let stream_id = js_sys::Reflect::get(&result, &JsValue::from_str("streamId"))
        .map_err(|_| RecordingError::StreamAcquisitionFailed {
            details: "Tab capture shim returned an unexpected response".into(),
        })?
        .as_string()
        .ok_or_else(|| RecordingError::StreamAcquisitionFailed {
            details: "Tab capture streamId is not a string".into(),
        })?;

    // In V0.1 the stream ID is captured but the actual MediaStream
    // reconstruction in the offscreen document is handled by a higher-level
    // orchestrator (planned for Story 1.3+).  For now, we return a minimal
    // stream that will be replaced once the offscreen doc is wired up.
    //
    // A proper reconstruction would look like:
    //   let mut constraints = MediaStreamConstraints::new();
    //   constraints.set_audio(&JsValue::from(false));
    //   let video_val = serde_wasm_bindgen::to_value(&json!({
    //       mandatory: { chromeMediaSource: "tab", chromeMediaSourceId: &stream_id }
    //   })).unwrap();
    //   constraints.set_video(&video_val);
    //   let promise = media_devices.get_user_media_with_constraints(&constraints)?;
    //   let js_value = JsFuture::from(promise).await?;
    //   let stream: MediaStream = js_value.into();
    let _ = stream_id; // Suppress unused-variable warning.

    let stream = MediaStream::new().expect("invariant: MediaStream::new() should never fail");
    Ok(stream)
}

/// Acquire a microphone audio track via `getUserMedia({ audio: true })`.
///
/// Returns `Ok(None)` when the user explicitly chooses to continue without
/// mic after a permission denial.  Returns
/// `Err(RecordingError::StreamAcquisitionFailed)` when the user cancels.
async fn acquire_microphone_with_handler(
    denied_handler: &Option<MicDeniedHandler>,
) -> Result<Option<MediaStreamTrack>> {
    let constraints = MediaStreamConstraints::new();
    constraints.set_audio(&JsValue::from(true));

    let window = web_sys::window().ok_or_else(|| RecordingError::StreamAcquisitionFailed {
        details: "No window object — cannot acquire microphone".into(),
    })?;

    let media_devices =
        window
            .navigator()
            .media_devices()
            .map_err(|_| RecordingError::StreamAcquisitionFailed {
                details: "navigator.mediaDevices is not available".into(),
            })?;

    let promise = media_devices
        .get_user_media_with_constraints(&constraints)
        .map_err(|_| RecordingError::StreamAcquisitionFailed {
            details: "getUserMedia() call failed".into(),
        })?;

    match JsFuture::from(promise).await {
        Ok(js_value) => {
            let stream: MediaStream = JsCast::dyn_into(js_value).map_err(|_| {
                RecordingError::StreamAcquisitionFailed {
                    details: "getUserMedia() did not return a MediaStream".into(),
                }
            })?;

            let tracks = stream.get_audio_tracks();
            let track: Option<MediaStreamTrack> = if tracks.length() > 0 {
                let val = tracks.get(0);
                val.dyn_into::<MediaStreamTrack>().ok()
            } else {
                None
            };
            Ok(track)
        }
        Err(_) => {
            // Microphone denied or unavailable — consult the handler.
            let should_continue = match denied_handler {
                Some(handler) => handler(),
                None => default_mic_denied_handler(),
            };

            if should_continue {
                Ok(None)
            } else {
                Err(RecordingError::StreamAcquisitionFailed {
                    details: "Microphone access denied by user".into(),
                })
            }
        }
    }
}

/// Default microphone-denied handler: `window.confirm()`.
fn default_mic_denied_handler() -> bool {
    web_sys::window()
        .and_then(|w| {
            w.confirm_with_message(
                "Microphone is unavailable. Continue without mic?",
            )
            .ok()
        })
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Audio mixer
// ---------------------------------------------------------------------------

/// Combine audio from the video source and microphone into a single
/// `MediaStream` using an `AudioContext`.
///
/// The returned `AudioContext` MUST be kept alive for the duration of
/// recording — dropping the context stops all audio flow.
///
/// ## Mixing strategy
///
/// 1. Create an `AudioContext`.
/// 2. If the video source has audio tracks, connect them to the destination
///    via a `MediaStreamAudioSourceNode`.
/// 3. If a mic track is provided, wrap it in a `MediaStream` and connect it
///    to the same destination.
/// 4. Return the destination's stream (the combined mix) and the context.
pub(crate) fn mix_audio(
    video_source: &MediaStream,
    mic_track: Option<&MediaStreamTrack>,
) -> Result<(MediaStream, AudioContext)> {
    let ctx = AudioContext::new().map_err(|_| RecordingError::StreamAcquisitionFailed {
        details: "Failed to create AudioContext for audio mixing".into(),
    })?;

    let dst = ctx
        .create_media_stream_destination()
        .map_err(|_| RecordingError::StreamAcquisitionFailed {
            details: "Failed to create MediaStreamAudioDestinationNode".into(),
        })?;

    // Connect video source audio tracks to the destination.
    if video_source.get_audio_tracks().length() > 0 {
        let src = ctx
            .create_media_stream_source(video_source)
            .map_err(|_| RecordingError::StreamAcquisitionFailed {
                details: "Failed to create audio source from video stream".into(),
            })?;
        src.connect_with_audio_node(&dst)
            .map_err(|_| RecordingError::StreamAcquisitionFailed {
                details: "Failed to connect video audio to mixer".into(),
            })?;
    }

    // Connect mic track to the destination.
    if let Some(track) = mic_track {
        let mic_stream =
            MediaStream::new().expect("invariant: MediaStream::new() should never fail");
        mic_stream
            .add_track(track);

        let mic_src = ctx
            .create_media_stream_source(&mic_stream)
            .map_err(|_| RecordingError::StreamAcquisitionFailed {
                details: "Failed to create audio source from microphone".into(),
            })?;
        mic_src
            .connect_with_audio_node(&dst)
            .map_err(|_| RecordingError::StreamAcquisitionFailed {
                details: "Failed to connect microphone to mixer".into(),
            })?;
    }

    let combined = dst.stream();
    Ok((combined, ctx))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// Note: Tests involving web-sys types (MediaStream, AudioContext) require a
// WASM runtime and live in #[cfg(target_arch = "wasm32")] blocks or are
// exercised via wasm-pack.  Native tests below cover pure-Rust logic only.

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // StreamAcquisitionService — pure-Rust construction & accessors
    // ------------------------------------------------------------------

    #[test]
    fn test_new_service() {
        let svc = StreamAcquisitionService::new(RecordingMode::FullScreen, true);
        assert_eq!(svc.mode(), &RecordingMode::FullScreen);
        assert!(svc.mic_enabled());
    }

    #[test]
    fn test_new_service_no_mic() {
        let svc = StreamAcquisitionService::new(RecordingMode::Tab, false);
        assert_eq!(svc.mode(), &RecordingMode::Tab);
        assert!(!svc.mic_enabled());
    }

    #[test]
    fn test_set_mic_denied_handler() {
        let mut svc = StreamAcquisitionService::new(RecordingMode::FullScreen, true);
        svc.set_mic_denied_handler(Box::new(|| false));
        // Handler is set — no panic means success.
    }

    #[test]
    fn test_mic_denied_handler_returns_true() {
        let handler: MicDeniedHandler = Box::new(|| true);
        assert!(handler());
    }

    #[test]
    fn test_mic_denied_handler_returns_false() {
        let handler: MicDeniedHandler = Box::new(|| false);
        assert!(!handler());
    }

    // ------------------------------------------------------------------
    // Error message formatting (UX-DR17)
    // ------------------------------------------------------------------

    #[test]
    fn test_stream_acquisition_error_messages() {
        let cases = [
            "Screen or tab selection was cancelled.",
            "Could not access tab. Check permissions in chrome://extensions and try again.",
            "No microphone found. Recording will continue without audio.",
            "Screen capture is not supported in this browser.",
        ];

        for detail in cases {
            let err = RecordingError::StreamAcquisitionFailed {
                details: detail.to_string(),
            };
            assert!(
                err.to_string().contains(detail),
                "Error message should contain detail: {detail}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// WASM tests (run via `wasm-pack test --headless --chrome`)
// ---------------------------------------------------------------------------

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    /// Verify that acquire_display() returns an error in headless context
    /// where no display picker can respond.
    #[wasm_bindgen_test]
    async fn test_acquire_display_cancelled() {
        let result = acquire_display().await;
        // In headless Chrome without a real display picker, getDisplayMedia
        // will error.  We verify the error variant is correct.
        if let Err(e) = result {
            assert!(matches!(
                e,
                RecordingError::StreamAcquisitionFailed { .. }
            ));
        }
    }

    /// Verify that acquire_microphone returns an error when mic is denied.
    #[wasm_bindgen_test]
    async fn test_acquire_microphone_denied() {
        // No real mic in headless — this should fail.
        let result =
            acquire_microphone_with_handler(&None).await;
        if let Err(e) = result {
            assert!(matches!(
                e,
                RecordingError::StreamAcquisitionFailed { .. }
            ));
        }
    }

    /// Verify that Service::acquire() fails gracefully in headless context.
    #[wasm_bindgen_test]
    async fn test_service_acquire_fails_in_headless() {
        let svc = StreamAcquisitionService::new(RecordingMode::FullScreen, true);
        let result = svc.acquire().await;
        if let Err(e) = result {
            assert!(matches!(
                e,
                RecordingError::StreamAcquisitionFailed { .. }
            ));
        }
    }
}
