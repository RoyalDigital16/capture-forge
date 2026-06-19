---
stepsCompleted: [1, 2, 3]
step01CompletedAt: 2026-06-19
step02CompletedAt: 2026-06-19
step03CompletedAt: 2026-06-19
inputDocuments:
  - _bmad-output/planning-artifacts/prds/prd-capture-forge-2026-06-19/prd.md
  - _bmad-output/planning-artifacts/architecture.md
  - _bmad-output/planning-artifacts/ux-designs/ux-capture-forge-2026-06-19/DESIGN.md
  - _bmad-output/planning-artifacts/ux-designs/ux-capture-forge-2026-06-19/EXPERIENCE.md
---

# capture-forge - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for capture-forge, decomposing the requirements from the PRD, UX Design, and Architecture requirements into implementable stories.

**Organising principle:** One epic = one primary user value + one primary risk boundary. Epics are ordered by dependency flow: each epic builds on its predecessors but must be independently testable and shipable.

## Requirements Inventory

### Functional Requirements

**Epic 1 — Recorder Core (V0.1, P0):**

| ID | Description | Source |
|----|-------------|--------|
| FR1 | Screen recording via `getDisplayMedia` (full desktop) | REC-01 |
| FR2 | Tab recording via `tabCapture` (specific tab) | REC-02 |
| FR3 | Microphone capture via AudioContext mixer (single mixed track) | REC-03 |
| FR4 | Pause/Resume with accurate duration tracking | REC-04 |
| FR5 | Stop recording and open preview page | REC-05 |
| FR6 | Cancel a recording-in-progress (returns to Idle) | REC-06 |
| FR7 | Visual 3-2-1 countdown overlay with circle animation | REC-07 |
| FR8 | WebM export via chunk concatenation (no re-encode) | REC-08 |
| FR9 | Basic crash recovery — detect orphan OPFS chunks, propose Restore via non-modal toast | REC-10 |
| FR10 | Minimal preview page — video player, Download, Delete | §6.1 |
| FR11 | RecorderStatusBar — elapsed timer (MM:SS), Pause and Stop controls during active recording | §6.4 |
| FR12 | Default keyboard shortcuts (Alt+Shift+G / M / X) | §6.1 |
| FR13 | Heartbeat keepalive — ping/pong every 20s from offscreen doc to SW | Architecture |

**Epic 2 — Resilient Storage & Recovery (V0.1→V0.2):**

| ID | Description | Source |
|----|-------------|--------|
| FR14 | OPFS storage with formal chunk lifecycle: `.partial` (Started) → `.written` (Written) → `.bin` (Committed) → Verified | REC-09 |
| FR15 | Session manifest (`manifest.json`) — per-chunk metadata (index, track, size, checksum, status, timestamp) | §6.6 |
| FR16 | Triple verification — manifest vs filesystem, file size vs manifest entry, chunk index contiguity | REC-A9 |
| FR17 | Integrity report (`integrity-report.json`) — recovered/lost summary with recommended actions | §6.6 |
| FR18 | Stale lock cleanup — lock >30s auto-cleaned at startup | REC-A6 |
| FR19 | IndexedDB fallback when OPFS is unavailable (V0.2) | REC-14 |
| FR20 | Storage manager — list sessions, delete recordings (V0.2) | REC-11 |
| FR21 | Quota display — estimated storage usage before starting a recording (V0.3) | REC-13 |

**Epic 3 — Recorder UX & Adoption Polish (V0.1→V0.2):**

| ID | Description | Source |
|----|-------------|--------|
| FR22 | Popup UI — mode selection (Full Screen / Tab), mic toggle, Start button | §6.4 |
| FR23 | Permission request handling — native Chrome dialogs for tabCapture / desktopCapture | §6.1 |
| FR24 | i18n — French locale (V0.1) | NFR-I18N-01 |
| FR25 | Configurable keyboard shortcuts — chrome.commands UI (V0.2) | REC-12 |
| FR26 | First-run permission onboarding page / setup wizard (V0.2) | §18 Phase 1 |

**Epic 4 — Overlay & Editor (P1, V0.5):**

| ID | Description | Source |
|----|-------------|--------|
| FR27 | Video player for recorded sessions (play/pause, seek, volume, fullscreen) | ED-01 |
| FR28 | Trim — start/end cut, non-destructive (metadata-only) | ED-02 |
| FR29 | Mute audio track | ED-03 |
| FR30 | Simple crop (visible area) | ED-04 |
| FR31 | Export after editing (WebM) | ED-05 |
| FR32 | Floating annotation toolbar — shadow DOM injection during recording | ED-06 |
| FR33 | Canvas annotation tools — pen, highlighter, text, shapes, arrow, blur | ED-06/07/08 |
| FR34 | Undo/Redo annotation history | ED-09 |
| FR35 | MP4 export via FFmpeg WASM | ED-11 |
| FR36 | GIF export via FFmpeg WASM | ED-11 |

**Epic 5 — Camera, Region & Firefox (P1→V1.0):**

| ID | Description | Source |
|----|-------------|--------|
| FR37 | Camera PiP overlay — select camera, resize, drag during recording | ED-10 |
| FR38 | Camera-only recording page (no screen/tab capture) | §7.1 |
| FR39 | Region selection page — select viewport area to record | §7.1 |
| FR40 | Firefox support — offscreen document → dedicated tab adaptation | §12 |
| FR41 | Firefox-specific permission model | §12 |
| FR42 | Firefox Add-ons store submission package | §18 Phase 3 |
| FR43 | i18n — 18 languages (V1.0) | NFR-I18N-02 |

**Epic 6 — AI & Enrichment (P2, V2.0+):**

| ID | Description | Source |
|----|-------------|--------|
| FR44 | Local STT transcription — sherpa-onnx Zipformer EN, model download to OPFS | §8.2 |
| FR45 | SRT/VTT subtitle export from transcription with VAD segment timing | §8.2 |
| FR46 | Cloud LLM integration (aisdk) — tutorial generation, auto-summary, smart search | §8.3 |
| FR47 | DOM capture — activeTab scope, privacy auto-mask, OPFS storage | §8.4 |

> **Note:** AudienceLens (§9) is an architectural north-star vision, not an implementation epic. It constrains today's storage format and message protocol decisions but has no deliverable scope in V0.1–V2.0.

### Non-Functional Requirements

**Performance:**

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-PERF-01 | Recording framerate | ≥25 FPS @ 1080p |
| NFR-PERF-02 | Audio sync tolerance (mic + screen) | <100ms drift |
| NFR-PERF-03 | RAM during recording | <500MB for 1h session |
| NFR-PERF-04 | WebM export (5min video) | <3s |
| NFR-PERF-05 | WASM load time | <1s from SW start to ready |
| NFR-PERF-06 | Canvas annotation latency | <16ms per stroke |
| NFR-PERF-07 | Chunk OPFS write overhead | <200ms per 10s chunk |
| NFR-PERF-08 | MP4 export (5min) | <2min (P1) |

**Reliability:**

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-REL-01 | Session uptime | 99% of sessions ≥1h complete without error |
| NFR-REL-02 | Crash recovery detection | 100% detection of orphaned OPFS chunks at startup |
| NFR-REL-03 | Data integrity after crash | 0% false positives in recovery |
| NFR-REL-04 | Chunk verification | Triple check (manifest vs fs, size, index contiguity) |
| NFR-REL-05 | Graceful degradation | Every failure produces a user-facing message with suggested action |

**Security:**

| ID | Requirement |
|----|-------------|
| NFR-SEC-01 | No data ever leaves browser except user-initiated downloads and optional API calls (P2) |
| NFR-SEC-02 | DOM capture (P2) disabled by default, activeTab only, auto-mask for sensitive fields |
| NFR-SEC-03 | Community audience lenses (P2+) sandboxed with declared CapabilitySet |
| NFR-SEC-04 | API keys stored in chrome.storage.local — never in extension code or localStorage |
| NFR-SEC-05 | All network requests (P2) go to user-configured endpoints — no hardcoded URLs |

**Accessibility (WCAG 2.1 AA):**

| ID | Requirement |
|----|-------------|
| NFR-A11Y-01 | All interactive elements have aria-label |
| NFR-A11Y-02 | Full keyboard navigation (Tab/Enter/Escape) |
| NFR-A11Y-03 | Color contrast ≥4.5:1 |
| NFR-A11Y-04 | Animations respect prefers-reduced-motion |
| NFR-A11Y-05 | Screen reader announcements for recording state changes (aria-live) |

**Internationalization:**

| ID | Requirement |
|----|-------------|
| NFR-I18N-01 | V0.1: English + French |
| NFR-I18N-02 | V1.0: 18 languages (en, fr, de, es, ca, hi, id, it, ko, pl, pt_BR, pt_PT, ru, ta, tr, uk, zh_CN, zh_TW) |

### Additional Requirements (Architecture)

- **Starter template**: Not applicable — Rust crate already scaffolded (Oxichrome v0.2 + Leptos v0.7). No CLI init needed.
- **2-WASM strategy**: `core.wasm` (recorder, storage, export, editor, overlay) + `ai.wasm` (sherpa-onnx STT, aisdk LLM, DOM capture — lazy-loaded P2+). AI cold-start never penalizes recording-only users.
- **Hybrid message-passing**: Direct Rust calls within core modules (recorder → storage → export). `ExtensionMessage` IPC for UI surfaces (popup, overlay, preview) via background message router.
- **Chunk binary format**: 32-byte header (magic "CFCH" + version + index + timestamp + size + XXH3 checksum) + raw MediaRecorder blob. Self-describing for recovery without manifest.
- **Heartbeat keepalive**: Offscreen-document ping/pong every 20s during active recording. 3 consecutive missed pings (60s) → assume SW dead → finalize chunk → CrashRecovery state.
- **Test strategy**: 3-tier pyramid — unit (`cargo test`, native, every commit), WASM (`wasm-pack test --headless --chrome`, CI nightly), E2E (Playwright, pre-release).
- **Feature flags**: `default = ["recorder", "storage", "export"]`. P1 features: `overlay`, `editor`, `camera`. P2 features: `stt`, `llm`, `dom`. Never compile P2 into default binary.
- **Implementation patterns**:
  - Every function returns `Result<T, RecordingError>` — no bare `unwrap()`
  - Exhaustive match on enums — no `_` catch-all without `unreachable!("reason")`
  - Every data-carrying type: `#[derive(Debug, Clone, Serialize, Deserialize)]`
  - `pub(crate)` by default, `pub` only for message boundary or external shims
  - `panic::set_hook()` override prevents WASM instance death from unexpected panics
- **Permissions V0.1**: `storage`, `unlimitedStorage`, `desktopCapture`, `tabCapture`, `downloads`
- **OPFS storage layout**: `capture-forge/sessions/<sessionId>/screen/chunk_*.bin + manifest.json + integrity-report.json`
- **chrome.storage.local**: `in_flight: SessionId | null` (active session lock)

### UX Design Requirements

**Design tokens & Visual system:**

| ID | Requirement | Source |
|----|-------------|--------|
| UX-DR1 | Color palette: light/dark pairs for background, foreground, muted, muted-foreground, primary, primary-foreground, accent, accent-foreground, destructive, border, ring, surface-overlay, recording-dot, countdown-fill, integrity-* states | DESIGN.md Colors |
| UX-DR2 | Typography system: body 13px/400, body-sm 11px/400, label 12px/500 (0.02em tracking), timer 20px/600 (mono, 0.05em), countdown 72px/700 (mono). System font stack only — no custom fonts V0.1. | DESIGN.md Typography |
| UX-DR3 | Spacing system: 4px unit scale (4, 8, 12, 16, 20, 24, 32, 40). Popup padding 16px, overlay padding 12px. | DESIGN.md Spacing |
| UX-DR4 | Shadow/elevation: toolbar shadow (8px blur, 2px y-offset), toast/dialog shadow (12px blur, 4px y-offset). No deeper than "surface/overlay/modal" 3-deep. | DESIGN.md Elevation |
| UX-DR5 | Rounded corners: sm 4px (inputs, badges), md 6px (buttons, cards, popup), lg 10px (toolbar, toast, countdown), full 9999px (integrity badges, recording dot). | DESIGN.md Shapes |

**Components:**

| ID | Requirement | Source |
|----|-------------|--------|
| UX-DR6 | Popup container: 280px fixed width, single-column vertically stacked, bg light/dark, padding 16px, radius lg | DESIGN.md |
| UX-DR7 | Mode selector: two-button radio group [Full Screen] [Tab], active fill primary / inactive fill muted, 32px height, md radius | DESIGN.md |
| UX-DR8 | Mic toggle: icon + label row, checked primary color, unchecked muted-foreground | DESIGN.md |
| UX-DR9 | Start button: primary fill, primary-foreground text, md radius, 36px height, full-width, disabled state opacity 0.5 | DESIGN.md |
| UX-DR10 | RecorderStatusBar: horizontal row centered, bg light/dark, lg radius, shadow, padding 12px, height 44px, min-width 180px. Timer + Pause + Stop controls | DESIGN + EXPERIENCE |
| UX-DR11 | Timer display: red (recording-dot) foreground, mono font 20px/600, MM:SS→HH:MM:SS, updates 250ms. Blinks (0.3↔1.0, 1s cycle) with "Paused" label during pause | EXPERIENCE |
| UX-DR12 | Pause/Stop icon buttons: 32x32, sm radius. Pause: foreground color. Stop: destructive color. Hover: muted bg. Pause toggle icon ⏸↔▶ | EXPERIENCE |
| UX-DR13 | Countdown overlay: full-viewport centered, semi-transparent bg 60%, number fill color, 72px/700 mono, circle ring stroke animated clockwise 1s/number. Scale-up+fade animation. Escape cancels | EXPERIENCE |
| UX-DR14 | Preview video player: 16:9 aspect ratio, black bg, browser-native \<video\> controls. Actions bar: [Download] primary button, [Delete] destructive outline button | EXPERIENCE |
| UX-DR15 | Crash recovery toast: non-modal bottom-center, bg light/dark, border, shadow-lg, radius md. Text: "A previous recording session was found." Actions: [Restore] primary / [Dismiss] text link. Auto-dismiss 8s. role="alert", aria-live="assertive" | EXPERIENCE |
| UX-DR16 | Integrity badge: 3 states — Clean (green), Partial (amber), Incomplete (red). Full radius, non-interactive label. Positioned above video player in preview | DESIGN + EXPERIENCE |

**Error states (UX-DR17):**

| Failure Mode | Message | Suggestion |
|-------------|---------|------------|
| Stream acquisition failed | "Could not access screen or tab." | "Check permissions in chrome://extensions and try again." |
| MediaRecorder error | "Recording stopped unexpectedly." | "Your recording was saved up to the interruption point. Try a shorter recording." |
| Export failed | "Could not create WebM file." | "Check available disk space and try again." |
| OPFS write error | "Could not save recording data." | "Storage may be full. Free up space and try again." |

**Interaction & Accessibility (UX-DR18):**

| Requirement | Details |
|-------------|---------|
| Mouse-first V0.1 | Keyboard shortcuts exist (Alt+Shift+G/M/X) but are not UI-discoverable. No shortcut config UI V0.1 |
| Keyboard navigation | Popup: Tab through mode selector → mic toggle → Start. Preview: Tab through player → Download → Delete |
| Screen reader | aria-live="polite" for state changes. aria-live="assertive" for countdown. role="alert" for toast |
| Reduced motion | Countdown circle fill respects prefers-reduced-motion (opacity fade). Timer blink slows to 2s cycle |
| Banned V0.1 | Drag-to-reposition toolbar, multi-select, hover-to-reveal controls, right-click context menu |
| Voice & tone | Neutral, precise, calm. No exclamation marks, no emoji, no celebration. Error messages name the problem and suggest a fix |
| Privacy gate | All NFR-SEC requirements are tested as acceptance gates in every relevant story, not just documented as NFRs |

### FR Coverage Map

```
FR1:  Epic 1 — Full-screen recording (getDisplayMedia)
FR2:  Epic 1 — Tab recording (tabCapture)
FR3:  Epic 1 — Microphone capture (AudioContext mixer)
FR4:  Epic 1 — Pause / Resume
FR5:  Epic 1 — Stop + preview
FR6:  Epic 1 — Cancel recording
FR7:  Epic 1 — 3-2-1 countdown
FR8:  Epic 1 — WebM export (chunk concat)
FR9:  Epic 1 — Basic crash recovery (orphan detection + toast)
FR10: Epic 1 — Minimal preview page
FR11: Epic 1 — RecorderStatusBar (timer, pause, stop)
FR12: Epic 1 — Default keyboard shortcuts
FR13: Epic 1 — Heartbeat keepalive

FR14: Epic 2 — OPFS chunk lifecycle
FR15: Epic 2 — Session manifest
FR16: Epic 2 — Triple verification
FR17: Epic 2 — Integrity report
FR18: Epic 2 — Stale lock cleanup
FR19: Epic 2 — IndexedDB fallback (V0.2)
FR20: Epic 2 — Storage manager (V0.2)
FR21: Epic 2 — Quota display (V0.3)

FR22: Epic 3 — Popup UI
FR23: Epic 3 — Permission handling
FR24: Epic 3 — i18n French (V0.1)
FR25: Epic 3 — Configurable shortcuts (V0.2)
FR26: Epic 3 — Onboarding wizard (V0.2)

FR27: Epic 4 — Video player
FR28: Epic 4 — Trim
FR29: Epic 4 — Mute
FR30: Epic 4 — Crop
FR31: Epic 4 — Export after edit
FR32: Epic 4 — Annotation toolbar
FR33: Epic 4 — Canvas annotation tools
FR34: Epic 4 — Undo/Redo
FR35: Epic 4 — MP4 export
FR36: Epic 4 — GIF export

FR37: Epic 5 — Camera PiP
FR38: Epic 5 — Camera-only page
FR39: Epic 5 — Region selection
FR40: Epic 5 — Firefox support
FR41: Epic 5 — Firefox permissions
FR42: Epic 5 — Firefox store submission
FR43: Epic 5 — i18n 18 languages

FR44: Epic 6 — Local STT
FR45: Epic 6 — SRT/VTT subtitles
FR46: Epic 6 — LLM integration
FR47: Epic 6 — DOM capture
```

## Epic List

### Epic 1: Recorder Core (V0.1, P0)

**Goal:** Users can record their full screen or a specific browser tab with microphone audio, pause and resume the recording, stop and preview the result, and export a WebM file. A basic crash recovery path detects orphaned chunks on restart and proposes restoration.

**User value delivered:** A complete record → preview → download loop in under 3 clicks. The extension is useful from day one without any peripheral features.

**FRs covered:** FR1, FR2, FR3, FR4, FR5, FR6, FR7, FR8, FR9, FR10, FR11, FR12, FR13

**Risk boundary:** Can we acquire a media stream, encode it via MediaRecorder, write chunks, and present a valid WebM to the user without crashing? This epic proves the core loop before any storage complexity is added.

**Key files:** `recorder/` (lifecycle, chunk, stream), `countdown.rs`, `export/webm.rs`, `preview.rs`, `background.rs`, `messaging.rs`, `error.rs`

### Story 1.1: Error System & State Machine Foundation

As a developer embedding the recorder,
I want a well-defined error system, state machine, and message protocol,
So that all recording operations have consistent error handling, valid state transitions are enforced, and UI surfaces can communicate with the core.

**Acceptance Criteria:**

**Given** the project has no error infrastructure yet
**When** a `RecordingError` enum is defined using `thiserror` with stable error codes (`StreamAcquisitionFailed`, `MediaRecorderError`, `WriteError`, `ExportError`, `StateViolation`, etc.) and a `details: String` field
**Then** every public function in the crate returns `Result<T, RecordingError>` via module-level `type Result<T> = std::result::Result<T, RecordingError>` aliases

**Given** the `SessionState` enum defines all V0.1 states (`Idle`, `Starting`, `Countdown`, `Recording`, `Paused`, `Stopping`, `Preview`, `Error`, `CrashRecovery`)
**When** the `RecordingSession` state machine is implemented with `transition()` enforcing valid moves (e.g., `Idle→Starting`, `Recording→Paused`, `Recording→Stopping`)
**Then** invalid transitions return `Err(RecordingError::StateViolation)` and the session state remains unchanged

**Given** the `ExtensionMessage` enum defines all V0.1 IPC variants (`StartRecording`, `StopRecording`, `PauseRecording`, `ResumeRecording`, `CancelRecording`, `VideoReady`, `RecordingError`, `KeepalivePing`, `KeepalivePong`, `GetStreamingData`, `ApplyStreamingData`)
**When** every variant derives `Debug, Clone, Serialize, Deserialize`
**Then** messages round-trip through serde JSON without data loss, verified by `cargo test`

**Given** a panic hook override installed at extension init
**When** a panic occurs in any core module
**Then** the hook logs the panic message via `console.error`, transitions the session to `Error` state, and does not abort the WASM instance

**Given** the state machine module, error module, and messaging module all have `#[cfg(test)] mod tests` blocks
**When** `cargo test` is executed
**Then** all state transitions are verified (`Idle→Starting→Recording→Paused→Recording→Stopping→Preview→Idle`, plus all error paths), all serde roundtrips pass, and edge cases (double-start, double-stop, pause-in-idle) return correct errors

---

### Story 1.2: Stream Acquisition — Screen, Tab & Mic

As a user,
I want to record my full screen or a specific browser tab with optional microphone audio,
So that I can capture the right source without relying on external tools.

**Acceptance Criteria:**

**Given** a user clicks Start with "Full Screen" mode selected
**When** the system requests a display stream via `getDisplayMedia()`
**Then** the browser display picker is shown
**And** the selected display is returned as a `MediaStream`
**And** if the picker is cancelled, `RecordingError::StreamAcquisitionFailed` is returned and the session returns to `Idle`

**Given** a user clicks Start with "Tab" mode selected
**When** the system acquires the selected tab stream through the Chrome tab capture flow
**Then** the selected tab stream is returned as a `MediaStream`
**And** if access is denied or acquisition fails, `RecordingError::StreamAcquisitionFailed` is returned with a user-facing message

**Given** the mic toggle is ON in the popup
**When** recording starts
**Then** microphone input is requested
**And** an `AudioContext` mixer combines source audio and microphone audio when both are available
**And** the recorder receives a single mixed audio track

**Given** the mic toggle is OFF
**When** recording starts
**Then** no microphone track is added to the recording stream
**And** available source audio is preserved if present

**Given** the mic toggle is ON
**When** microphone permission is denied
**Then** the system shows a confirmation dialog explaining that microphone audio is unavailable
**And** the user can choose "Continue without mic" or "Cancel"
**And** if the user continues, recording starts without a microphone track
**And** if the user cancels, the session returns to `Idle`

**Given** stream acquisition dependencies are tested with mocked browser APIs
**When** recorder acquisition tests are executed
**Then** success, cancellation, denied permission, and missing-audio cases are covered and validated

---

### Story 1.3: Recording Lifecycle — Start, Stop, Pause, Resume, Cancel

As a user,
I want to control my recording with Start, Pause/Resume, Stop, and Cancel,
So that I can capture exactly the content I need without wasting storage on unwanted segments.

**Acceptance Criteria:**

**Given** a `MediaStream` has been acquired by Story 1.2
**When** `startRecording()` is called
**Then** a `MediaRecorder` is created from the selected stream with VP8+Opus MIME type
**And** the session transitions `Idle → Starting → Countdown → Recording` (countdown state supported here, animation overlay rendered by Story 1.6)
**And** the lifecycle implementation remains compatible with the existing performance benchmark suite

**Given** an active recording session
**When** `pauseRecording()` is called
**Then** `MediaRecorder.pause()` is invoked
**And** the session transitions to `Paused`
**And** duration tracking pauses while accumulated duration is preserved

**Given** a paused session
**When** `resumeRecording()` is called
**Then** `MediaRecorder.resume()` is invoked
**And** the session transitions back to `Recording`
**And** the timer resumes from the accumulated duration

**Given** an active recording session
**When** `stopRecording()` is called
**Then** `MediaRecorder.stop()` is invoked
**And** the final `ondataavailable` event is consumed
**And** the session transitions to `Stopping`

**Given** a session in `Starting` or `Countdown`
**When** `cancelRecording()` is called
**Then** no chunks are written
**And** no preview is produced
**And** the session returns directly to `Idle`

**Given** a session in `Recording`
**When** `cancelRecording()` is called
**Then** existing chunks are discarded
**And** the recording is not exported
**And** session resources are released before returning to `Idle`

**Given** the lifecycle module is under test
**When** the transition test suite is executed
**Then** all valid transitions succeed
**And** invalid transitions (`pause` in `Idle`, `resume` in `Idle`, `stop` in `Idle`, `start` in `Recording`, `cancel` in `Idle`) return `RecordingError::StateViolation`
**And** pause/resume cycles preserve total recorded duration

---

### Story 1.4: Chunk Writer Foundation

As a developer,
I want a chunk writer that buffers MediaRecorder output, prepends a fixed binary header, and writes chunks through a defined OPFS lifecycle,
So that recording data is persisted incrementally and can be validated independently from export.

**Acceptance Criteria:**

**Given** MediaRecorder emits an `ondataavailable` blob at the configured chunk interval
**When** the chunk writer receives the blob
**Then** it prepends a 32-byte header with the following layout:
- bytes 0–3: magic `CFCH`
- byte 4: version `0x01`
- bytes 5–8: chunk index `u32 LE`
- bytes 9–16: timestamp `f64 LE`
- bytes 17–24: payload size `u64 LE`
- bytes 25–28: XXH3 checksum `u32 LE`
- bytes 29–31: reserved (zero)
**And** writes the result to OPFS as `chunk_{index:06}.partial`

**Given** a chunk write completes successfully
**When** the file has been flushed and its expected size (32 + payload) has been validated
**Then** the file is renamed from `.partial` to `.written`
**And** after manifest-level commit acknowledgment it is renamed to `.bin`

**Given** the chunk writer is initialized
**When** the first chunk is written
**Then** session metadata for chunk index, size, and checksum is recorded for the active session (in-memory manifest entry, ready to be persisted by the storage/recovery layer)

**Given** an OPFS write fails during chunk persistence
**When** the writer detects the failure
**Then** the chunk remains in `.partial` state
**And** `RecordingError::WriteError` is returned with storage context
**And** no `.written` or `.bin` promotion occurs

**Given** the chunk writer test suite is executed
**When** header serialization, lifecycle transitions, and corruption detection are tested
**Then** header encode/decode roundtrips succeed
**And** lifecycle transitions `.partial → .written → .bin` are validated with mocked OPFS
**And** checksum verification detects corrupted payloads

---

### Story 1.5: WebM Export Pipeline

As a user,
I want my recorded chunks to be assembled into a valid WebM file,
So that I can play, share, and archive the recording.

**Acceptance Criteria:**

**Given** all chunks for a session are in `.bin` state
**When** the export pipeline reads the chunks in index order
**Then** each chunk header is validated for magic, version, checksum, index contiguity, and payload size
**And** the validated chunk payloads are assembled into a single WebM blob
**And** the resulting blob is created with MIME type `video/webm`

**Given** `concat()` is called
**When** a chunk has a corrupted checksum or missing/invalid header
**Then** `RecordingError::ExportError` is returned with the chunk index and failure reason

**Given** the session contains no exportable chunks
**When** export is requested
**Then** `RecordingError::ExportError` is returned with `EmptySession` reason

**Given** a 5-minute recording is exported
**When** measured from export call to blob ready
**Then** the wall-clock time meets the NFR benchmark target of under 3 seconds

**Given** the export test suite is executed
**When** valid, empty, and corrupted chunk sequences are tested
**Then** valid sequences produce a playable WebM blob
**And** empty and corrupted sequences return appropriate errors

---

### Story 1.6: Countdown & Recorder Status Bar

As a user,
I want to see a clear 3-2-1 countdown before recording starts and a persistent status bar during recording,
So that I know exactly when capture begins and can monitor recording state at a glance.

**Acceptance Criteria:**

**Given** the session transitions to `Countdown` state
**When** the countdown sequence begins
**Then** a full-viewport semi-transparent overlay (60% opacity) is rendered
**And** the sequence displays 3 → 2 → 1, each number for 1s with scale-up + fade animation
**And** a circle ring fills clockwise over each 1s interval
**And** respecting `prefers-reduced-motion` (opacity fade replaces scale animation)

**Given** the countdown overlay is visible
**When** the user presses `Escape`
**Then** the countdown is cancelled
**And** the session returns to `Idle`

**Given** the countdown completes without interruption
**When** the last number fades out
**Then** the session transitions to `Recording`

**Given** the session is in `Recording` state
**When** the RecorderStatusBar is rendered
**Then** it shows: elapsed timer (left), Pause button (center), Stop button (right)
**And** the timer uses `MM:SS` format, mono font, red foreground, updating every 250ms
**And** the bar has a semi-transparent background, 44px height, top-fixed position

**Given** the session transitions to `Paused`
**When** the status bar reflects the pause
**Then** the timer blinks (opacity 0.3 ↔ 1.0, 1s cycle, slowed to 2s with `prefers-reduced-motion`)
**And** a "Paused" label appears to the right of the timer
**And** the Pause icon switches to Resume (▶)

**Given** a Resume action occurs
**When** the status bar updates
**Then** the timer stops blinking
**And** the "Paused" label is removed
**And** the icon switches back to Pause (⏸)

---

### Story 1.7: Preview Page — Play, Download, Delete

As a user,
I want to preview my recording after stopping, then download or delete it,
So that I can confirm the result is correct before deciding what to do with it.

**Acceptance Criteria:**

**Given** the session transitions to `Preview` state
**When** the preview page opens
**Then** it displays a `<video>` element with 16:9 aspect ratio and browser-native controls
**And** the exported WebM blob is bound as the video source
**And** two action buttons are shown below the player: [Download] (primary) and [Delete] (destructive outline)

**Given** the preview page is open
**When** the user clicks Download
**Then** `chrome.downloads.download()` is invoked with the exported WebM file
**And** the browser download flow is triggered

**Given** the preview page is open
**When** the user clicks Delete
**Then** a confirmation dialog is shown: "Delete this recording?" [Cancel] [Delete]
**And** if confirmed, the session chunks, manifest, and preview references are removed
**And** the session returns to `Idle`
**And** if cancelled, the preview page remains open

**Given** the video player is focused
**When** the user presses `Space`
**Then** playback toggles between play and pause

**Given** the preview page is open and no blocking dialog is active
**When** the user presses `Escape`
**Then** the preview page closes and the session returns to `Idle`

**Given** the recording has known integrity issues after recovery
**When** the preview is displayed
**Then** an integrity badge is shown above the player
**And** the badge indicates the current state as Clean, Partial, or Incomplete
**And** preview and download remain available regardless of badge state

---

### Story 1.8: Crash Recovery Detection & Restore

As a user,
I want to be offered restoration of a previous recording if the extension recovers from a crash,
So that I do not lose my work even when something goes wrong.

**Acceptance Criteria:**

**Given** the extension starts (service worker init, popup open, or background wake)
**When** OPFS is scanned and orphaned chunks or manifests are found
**Then** a crash recovery event is raised
**And** a non-modal toast is shown: "A previous recording session was found."
**With** actions [Restore] (primary) and [Dismiss] (text link)
**And** the toast auto-dismisses after 8s of no interaction

**Given** the crash recovery toast is visible
**When** the user clicks Restore
**Then** the orphaned chunks are assembled using the available manifest
**And** the preview page opens with recovered content
**And** an integrity report is generated documenting what was recovered

**Given** the crash recovery toast is visible
**When** the user clicks Dismiss or 8s passes
**Then** the toast is removed
**And** the orphaned chunks remain on disk (opfs-cleanup handles later)
**And** the session returns to `Idle`

**Given** a crash recovery detection test
**When** the service worker is killed mid-recording and restarted
**Then** recovery is proposed with ≥90% session reconstruction success rate (NFR-REL-02)

---

### Story 1.9: Heartbeat Keepalive

As a developer,
I want a keepalive mechanism between the offscreen document and service worker,
So that the service worker stays alive during active recording and can detect a dead worker.

**Acceptance Criteria:**

**Given** a recording session is active
**When** the offscreen document is created
**Then** a `setInterval` at 20s starts sending `ExtensionMessage::KeepalivePing` to the service worker

**Given** the service worker receives a `KeepalivePing`
**When** it processes the message
**Then** it responds with `KeepalivePong`
**And** any message receipt resets Chrome's SW idle timer

**Given** 3 consecutive pings go unanswered (60s timeout)
**When** the offscreen document detects the absence of responses
**Then** it assumes the service worker is dead
**And** finalises the current chunk
**And** transitions to `CrashRecovery` state

**Given** the recording stops normally
**When** the recording lifecycle ends
**Then** the offscreen document clears the ping interval
**And** no further keepalive messages are sent

---

### Story 1.10: Keyboard Shortcuts

As a power user,
I want default keyboard shortcuts to start, stop, and pause a recording,
So that I can control the extension without clicking the popup.

**Acceptance Criteria:**

**Given** the extension is installed
**When** the shortcuts are registered via `chrome.commands`
**Then** `Alt+Shift+G` starts recording
**And** `Alt+Shift+X` cancels / stops recording
**And** `Alt+Shift+M` toggles pause / resume

**Given** a keyboard shortcut fires
**When** the associated action is processed
**Then** the same business logic is invoked as if the action was triggered from the UI
**And** errors are surfaced via the standard error path

**Given** a shortcut fires in an invalid state (e.g., `Alt+Shift+G` during active recording)
**When** the action cannot be executed
**Then** the event is silently ignored (no error dialog)
**And** the session state is unchanged

---

### Epic 2: Resilient Storage & Recovery (V0.1→V0.2)

**Goal:** Recordings are persisted reliably in OPFS with a formal chunk lifecycle, self-describing session manifest, triple verification on recovery, and an integrity report. Stale locks are cleaned automatically. In V0.2, IndexedDB fallback and a storage manager UI give users visibility and control over storage.

**User value delivered:** No data loss. Every session can be recovered after a crash. Users can manage their recordings and understand storage health.

**FRs covered:** FR14, FR15, FR16, FR17, FR18, FR19 (V0.2), FR20 (V0.2), FR21 (V0.3)

**Risk boundary:** OPFS reliability at scale. Can we write thousands of chunks, recover from interrupted writes, and give honest integrity reports? This epic is independent of the recorder core's stream acquisition logic.

**Key files:** `storage/opfs.rs`, `storage/recovery.rs`, `storage/indexdb.rs` (V0.2), `chunk.rs`

> **Rescope note vs PRD source :** REC-13 (quota display) and REC-14 (IndexedDB fallback) are listed in the PRD under V0.1 scope. The deliberate rescope to V0.2/V0.3 here reflects a product decision to prioritise recording reliability and recovery over storage management UX in the initial release. IndexedDB adds test surface without user-facing value at launch; quota display depends on a stable storage baseline.

### Story 2.1: Session Manifest & Storage Layout

As a developer,
I want a defined OPFS directory layout and a session manifest that records per-chunk metadata,
So that sessions are self-describing and recoverable without external files.

**Acceptance Criteria:**

**Given** a recording session is created
**When** the storage module initialises
**Then** an OPFS directory is created at `capture-forge/sessions/<sessionId>/screen/`
**And** a `manifest.json` file is initialised with session metadata (sessionId, start timestamp, recording mode, chunk count)

**Given** a chunk is committed (`.bin` status)
**When** the manifest is updated
**Then** it records: chunk index, file size, XXH3 checksum, track label, status, and commit timestamp

**Given** the session ends normally
**When** the manifest is finalised
**Then** the manifest is flushed to OPFS and closed
**And** `chrome.storage.local` clears `in_flight` lock

**Given** the manifest test suite is executed
**When** create, update, and finalise cycles are tested
**Then** manifest JSON roundtrips without data loss
**And** malformed manifests are detected and reported

### Story 2.2: Triple Verification & Integrity Report

As a developer,
I want to verify that stored chunks match the manifest, have valid sizes, and form a contiguous sequence,
So that recovery decisions are based on reliable data.

**Acceptance Criteria:**

**Given** a crash recovery event is raised
**When** triple verification runs
**Then** check 1 (manifest vs filesystem): every committed chunk in the manifest has a corresponding `.bin` file on OPFS
**And** check 2 (size match): each file's actual size matches the manifest entry
**And** check 3 (index contiguity): the longest prefix from index 0 with no gaps is identified

**Given** triple verification completes
**When** the results are evaluated
**Then** an `integrity-report.json` is generated with: total chunks, verified count, lost count, contiguous prefix length, and recommended action (restore / partial / abandon)

**Given** all three checks pass for a session
**When** the integrity report is generated
**Then** the report status is Clean (green)

**Given** only a contiguous prefix passes (some trailing chunks lost)
**When** the integrity report is generated
**Then** the report status is Partial (amber) and the recoverable range is documented

**Given** no usable prefix can be reconstructed
**When** the integrity report is generated
**Then** the report status is Incomplete (red) and the user is advised the session cannot be recovered

**Given** the recovery test suite is executed
**When** clean, partial, and incomplete scenarios are tested
**Then** NFR-REL-02 (100% orphan detection) and NFR-REL-03 (0% false positives) are verified

### Story 2.3: Stale Lock Cleanup

As a developer,
I want automatic cleanup of session locks older than 30 seconds,
So that a crashed session does not permanently block new recordings.

**Acceptance Criteria:**

**Given** the extension initialises
**When** `chrome.storage.local` contains an `in_flight` lock with a timestamp older than 30s
**Then** the lock is auto-cleared
**And** the session is treated as potentially crashed (Story 1.8 recovery scan may follow)

**Given** the extension initialises
**When** `in_flight` lock is younger than 30s
**Then** the lock is preserved and recovery check proceeds normally

**Given** the stale lock test suite is executed
**When** fresh, stale, and absent locks are simulated
**Then** stale locks are always cleaned and fresh locks are never touched

### Story 2.4: IndexedDB Fallback (V0.2)

As a user,
I want recordings to fall back to IndexedDB when OPFS is unavailable,
So that I never lose a recording due to storage API limitations.

**Acceptance Criteria:**

**Given** OPFS initialisation fails or is unavailable
**When** a recording session starts
**Then** the storage layer falls back to IndexedDB transparently
**And** the same chunk lifecycle (`.partial → .written → .bin`) is applied
**And** no user-facing error is shown for the fallback

**Given** OPFS becomes available after a session using IndexedDB
**When** the next session starts
**Then** OPFS is used as primary storage again

### Story 2.5: Storage Manager UI (V0.2)

As a user,
I want to see my recorded sessions, their storage usage, and delete unwanted ones,
So that I can manage disk space and keep my recordings organised.

**Acceptance Criteria:**

**Given** the popup or a dedicated page
**When** the user opens the storage manager
**Then** all sessions stored in OPFS are listed with: date, duration, file size, integrity status
**And** each session has a Delete action

**Given** a session is selected for deletion
**When** the user confirms deletion
**Then** the session directory (`sessions/<sessionId>/`) and manifest are removed from OPFS

### Story 2.6: Quota Display (V0.2→V0.3)

As a user,
I want to see estimated free storage before starting a recording,
So that I know whether I have enough space for the session.

**Acceptance Criteria:**

**Given** the popup is open and Idle
**When** storage is queried
**Then** an estimated available space is shown (or a "Storage full" warning if critical)
**And** the estimate refreshes each time the popup opens

---

### Epic 3: Recorder UX & Adoption Polish (V0.1→V0.2)

**Goal:** The popup UI, permission flows, French locale, and onboarding experience make the extension accessible and trustworthy. Configurable keyboard shortcuts (V0.2) power-user workflow.

**User value delivered:** A polished first-run experience, language accessibility for French users, and power-user shortcut customization.

**FRs covered:** FR22, FR23, FR24 (V0.1), FR25 (V0.2), FR26 (V0.2)

**Risk boundary:** Permission UX — can users successfully grant tabCapture/desktopCapture without confusion? i18n integration — does the i18n crate compose cleanly with Leptos CSR?

**Key files:** `popup.rs`, `permissions.rs`, `setup.rs`, `static/locales/en.json`, `static/locales/fr.json`

### Story 3.1: Popup UI — Mode Selection, Mic Toggle, Start Button

As a user,
I want a clear popup to choose my recording mode, toggle the microphone, and start recording,
So that I can begin capturing with minimal friction.

**Acceptance Criteria:**

**Given** the extension icon is clicked
**When** no recording is active
**Then** the popup opens at 280px width, single-column, with the CaptureForge visual identity (design tokens per UX-DR6 to UX-DR9)

**Given** the popup is in `Idle` state
**When** it renders
**Then** it displays:
- Mode selector: two-button radio group [Full Screen] [Tab], 32px height, default "Full Screen" selected
- Mic toggle: icon + label row, default ON
- Start button: primary fill, 36px height, full-width, label per typography system

**Given** the popup is in `Idle` state
**When** a mode is selected
**Then** the selection persists for the session lifetime
**And** if the popup closes and reopens, it resets to "Full Screen"

**Given** the Start button is clicked
**When** the session transitions to `Starting`
**Then** the popup shows a spinner with "Preparing…"
**And** the button is disabled during `Starting` and `Countdown` states

**Given** an error occurs during setup
**When** the session transitions to `Error`
**Then** the popup displays the error message and suggestion (per UX-DR17)
**And** a "Back" or "Close" action returns the session to `Idle`

### Story 3.2: Permission Request Handling

As a user,
I want clear permission prompts when I try to record,
So that I understand what access the extension needs and can grant or deny it.

**Acceptance Criteria:**

**Given** the user clicks Start with "Full Screen" mode selected
**When** the system initiates stream acquisition
**Then** Chrome's native `getDisplayMedia` dialog is shown with screen selection options
**And** if the user selects a screen, the permission is granted for the session

**Given** the user clicks Start with "Tab" mode selected
**When** the system initiates stream acquisition
**Then** a `chrome.tabCapture` permission request is triggered
**And** if granted, recording proceeds normally

**Given** a permission is denied
**When** the dialog is cancelled
**Then** the popup shows the error state per UX-DR17
**And** a suggestion is shown: "Check permissions in chrome://extensions and try again."

**Given** the mic is ON
**When** microphone permission is denied
**Then** the system shows a confirmation dialog (per Story 1.2 decision: "Continue without mic" or "Cancel")
**And** the popup reflects the mic state accurately

### Story 3.3: i18n — French Locale (V0.1)

As a French-speaking user,
I want the extension UI in French,
So that I can use it comfortably in my native language.

**Acceptance Criteria:**

**Given** the `i18n` crate is integrated
**When** locale files are loaded
**Then** `static/locales/en.json` contains all V0.1 UI strings (popup, status bar, countdown, preview, recovery toast, error messages)
**And** `static/locales/fr.json` provides complete French translations for the same strings

**Given** the browser language is set to French
**When** the extension loads
**Then** French locale is applied automatically
**And** all UI surfaces render in French

**Given** the browser language is English or unknown
**When** the extension loads
**Then** English locale is used as default fallback

### Story 3.4: Configurable Keyboard Shortcuts (V0.2)

As a power user,
I want to customise the keyboard shortcuts for recording actions,
So that I can match them to my personal workflow.

**Acceptance Criteria:**

**Given** the extension settings or `chrome://extensions/shortcuts`
**When** the user navigates to keyboard shortcut configuration
**Then** all three default shortcuts (Start, Stop/Cancel, Pause/Resume) are listed and editable

### Story 3.5: First-Run Permission Onboarding (V0.2)

As a new user,
I want a brief explanation of the permissions the extension needs when I first open it,
So that I understand why each permission is required before granting it.

**Acceptance Criteria:**

**Given** the extension is installed and opened for the first time
**When** the popup opens
**Then** a brief one-page explanation is shown listing: tabCapture, desktopCapture, storage, downloads, and why each is needed
**And** the user can dismiss the page and proceed to the main popup

---

### Epic 4: Overlay & Editor (P1, V0.5)

**Goal:** During recording, users can annotate the screen with a floating toolbar (pen, highlighter, text, shapes, blur). After recording, they can play back, trim, mute, crop, and export the result in WebM, MP4, or GIF.

**User value delivered:** Users can communicate visually during recording and polish the result without leaving the browser. Full annotation + trim + export pipeline.

**FRs covered:** FR27, FR28, FR29, FR30, FR31, FR32, FR33, FR34, FR35, FR36

**Risk boundary:** Canvas performance (<16ms per stroke), FFmpeg WASM binary size and cold-start, non-destructive editor architecture. This epic shares no core files with Epic 1–3 (content script + editor module).

**Key files:** `content_script/` (overlay, canvas/tools, canvas/history), `editor/` (player, operations, export), `js/ffmpeg.js`

### Story 4.1: Floating Annotation Toolbar

As a user,
I want a floating toolbar to appear during recording with annotation controls,
So that I can mark up the screen without switching applications.

**Acceptance Criteria:**

**Given** recording is active (P1 overlay mode)
**When** the content script injects the toolbar into the active tab
**Then** a floating toolbar appears, fixed at top-center, rendered via shadow DOM (no page style interference)
**And** it contains tool buttons: pen, highlighter, text, shapes, arrow, blur, colour picker, undo, redo
**And** the toolbar is semi-transparent at rest, full opacity on hover

### Story 4.2: Canvas Annotation Tools

As a user,
I want to draw, highlight, add text, shapes, and blur sensitive areas on the recorded page,
So that I can communicate visually during demos and bug reports.

**Acceptance Criteria:**

**Given** an annotation tool is selected from the toolbar (e.g., pen)
**When** the user draws on the page
**Then** strokes are rendered via Canvas 2D at <16ms per frame (NFR-PERF-06)
**And** the full set of tools is available: pen (variable width), highlighter (semi-transparent), text box, rectangle, arrow, blur (gaussian)

### Story 4.3: Undo/Redo Annotation History

As a user,
I want to undo or redo my annotations,
So that I can correct mistakes without restarting the recording.

**Acceptance Criteria:**

**Given** the user has drawn multiple annotations
**When** Undo is triggered
**Then** the last stroke is removed from the canvas
**When** Redo is triggered
**Then** the previously undone stroke is restored
**And** the history is cleared when the recording stops

### Story 4.4: Video Player for Recorded Sessions

As a user,
I want to play back my recording in a dedicated video player,
So that I can review the content before editing or exporting.

**Acceptance Criteria:**

**Given** a completed recording
**When** the editor opens
**Then** a `<video>` element with browser-native controls displays the recording
**And** the player supports play/pause, seek, volume, and fullscreen

### Story 4.5: Non-Destructive Trim, Mute & Crop

As a user,
I want to trim the start and end, mute audio, or crop the visible area of my recording,
So that I can remove unwanted sections and protect private information.

**Acceptance Criteria:**

**Given** a recording is loaded in the editor
**When** the user sets trim points and applies mute or crop
**Then** the operations are stored as `EditorSession` metadata — the source chunks are never modified
**And** trim respects the exact start/end boundaries; crop records the visible region

### Story 4.6: Export After Editing

As a user,
I want to export my edited recording as a final WebM file,
So that I can share the polished result.

**Acceptance Criteria:**

**Given** an `EditorSession` with trim/mute/crop applied
**When** export is triggered
**Then** a WebM blob is produced with the edits applied
**And** NFR-PERF-04 target is maintained for edited exports

### Story 4.7: MP4 Export (FFmpeg WASM)

As a user,
I want to export my recording as MP4,
So that I can upload it to platforms that don't support WebM.

**Acceptance Criteria:**

**Given** a completed recording (raw or edited)
**When** MP4 export is selected
**Then** the FFmpeg WASM JS shim transcodes the WebM to H.264+AAC MP4
**And** the export completes in <2min for a 5-minute video (NFR-PERF-08)

### Story 4.8: GIF Export (FFmpeg WASM)

As a user,
I want to export a short segment of my recording as an animated GIF,
So that I can embed it in PRs, documentation, or bug reports.

**Acceptance Criteria:**

**Given** a completed recording
**When** GIF export is selected with a duration selector
**Then** FFmpeg WASM produces an animated GIF from the selected segment

---

### Epic 5: Camera, Region & Firefox (P1→V1.0)

**Goal:** Users can overlay their webcam in picture-in-picture mode, record only their camera, or select a specific viewport region. The extension works on Firefox with equivalent functionality. Available in 18 languages.

**User value delivered:** Camera-in-recording for trainers, region recording for focused demos, Firefox users can install from their store.

**FRs covered:** FR37, FR38, FR39, FR40, FR41, FR42, FR43

**Risk boundary:** Firefox MV3 differs from Chrome — offscreen document replaced by dedicated tab, different permission model, different E2E testing. Region recording adds UI complexity for selection. Camera PiP requires MediaPipe interop or similar.

**Key files:** `camera_page.rs`, `region_page.rs`, `content_script/camera.rs`, Firefox adaptation in `background.rs`, `static/locales/*.json`

### Story 5.1: Camera PiP Overlay

As a user,
I want to overlay my webcam feed in picture-in-picture mode during recording,
So that viewers can see my reactions alongside the screen capture.

**Acceptance Criteria:**

**Given** recording is active
**When** the user activates camera PiP
**Then** a camera feed is displayed as a draggable, resizable overlay within the recording viewport
**And** it is composited into the final recording output

### Story 5.2: Camera-Only Recording Page

As a user,
I want to record only my webcam without screen capture,
So that I can create talking-head videos or quick personal messages.

**Acceptance Criteria:**

**Given** the camera-only mode is selected
**When** recording starts
**Then** only the camera feed is captured, with optional microphone audio
**And** no screen or tab stream is requested

### Story 5.3: Region Selection Page

As a user,
I want to select a specific region of my screen to record,
So that I capture only the relevant area and reduce file size.

**Acceptance Criteria:**

**Given** region mode is selected
**When** the user is prompted to select an area
**Then** a fullscreen overlay allows click-and-drag selection
**And** only the selected viewport rectangle is recorded

### Story 5.4: Firefox Support — Offscreen to Tab Adaptation

As a Firefox user,
I want to record screen and tab content,
So that I can use CaptureForge without switching to Chrome.

**Acceptance Criteria:**

**Given** the extension is loaded in Firefox
**When** recording starts
**Then** the offscreen document pattern is replaced by a dedicated background tab
**And** all core V0.1 recording features work equivalently on Firefox

### Story 5.5: Firefox Permission Model & Store Submission

As a Firefox user,
I want correct permission prompts and a store-compatible package,
So that I can install and use the extension from Firefox Add-ons.

**Acceptance Criteria:**

**Given** a Firefox build target
**When** the package is built
**Then** the manifest matches Firefox's MV3 requirements (or MV2 as needed)
**And** the extension passes Firefox Add-ons review guidelines

### Story 5.6: i18n — 18 Languages

As a non-English speaker,
I want the extension in my language,
So that I can use it comfortably in my native language.

**Acceptance Criteria:**

**Given** the extension has English and French locales
**When** 16 additional locale files are added
**Then** all 18 locales (NFR-I18N-02) are available in `static/locales/`
**And** the i18n system applies the correct locale automatically

---

### Epic 6: AI & Enrichment (P2, V2.0+)

**Goal:** Users can generate transcripts and subtitles from recordings locally via sherpa-onnx, optionally enhance with cloud LLM (aisdk) for tutorials and summaries, and capture DOM snapshots with privacy guards.

**User value delivered:** Searchable, subtitled, summarised recordings. Accessibility for hearing-impaired viewers. Tutorial generation from live sessions.

**FRs covered:** FR44, FR45, FR46, FR47

**Risk boundary:** WASM binary size for sherpa-onnx (~20MB model), model download UX, async initialization in constrained WASM runtime. Entirely feature-gated and lazy-loaded — never affects core cold-start.

**Key files:** `ai/` (transcription, captions, docgen, dom_capture) — separate ai.wasm, lazy-loaded on demand

> **Note on AudienceLens (§9):** The AudienceLens vision constrains today's storage format and message protocol (immutable source tracks, multi-track manifest compatibility, extensible message router) but is deliberately excluded from this epic's implementation scope. It becomes active as a design reference when P2 begins.

### Story 6.1: Local STT Transcription — sherpa-onnx

As a user,
I want my recording's speech transcribed locally on my machine,
So that I can search, caption, or repurpose the spoken content without uploading audio anywhere.

**Acceptance Criteria:**

**Given** the P2 feature flag `stt` is enabled
**When** the user triggers transcription
**Then** the sherpa-onnx Zipformer EN model is downloaded to OPFS at first use
**And** transcription runs entirely in-browser with VAD-based segment timing
**And** RAM stays <200MB for a 30-minute recording

### Story 6.2: SRT/VTT Subtitle Export

As a user,
I want to export subtitles from my recording's transcription,
So that I can add captions to the video for accessibility or multi-language use.

**Acceptance Criteria:**

**Given** a transcription is complete
**When** subtitle export is requested
**Then** SRT and VTT files are generated from the segment timing
**And** the user can download the subtitle file

### Story 6.3: Cloud LLM Integration — aisdk

As a user,
I want to generate tutorials, summaries, and search the content of my recording via LLM,
So that I can repurpose recordings into documentation or find specific moments quickly.

**Acceptance Criteria:**

**Given** the P2 feature flag `llm` is enabled and an API key is configured
**When** the user selects a generation action (tutorial, summary, search)
**Then** the aisdk crate sends the transcription/context to the configured provider
**And** the result is displayed in the UI

### Story 6.4: DOM Capture with Privacy Filters

As a user,
I want to capture the DOM state of the recorded page alongside the video,
So that I can replay exact page interactions or generate DOM-based documentation.

**Acceptance Criteria:**

**Given** the P2 feature flag `dom` is enabled
**When** DOM capture is activated
**Then** a snapshot of the active tab's DOM is captured (activeTab scope only)
**And** sensitive fields (password, credit card) are auto-masked
**And** the snapshot is stored in OPFS linked to the session

---

## NFR Cross-Epic Mapping

| NFR Category | Applies to Epics | How Enforced |
|-------------|-----------------|--------------|
| PERF-01 to PERF-08 | 1, 2, 4 | Benchmarks in CI per commit / per release |
| REL-01 to REL-05 | 1, 2 | Recovery integration tests, stress tests |
| SEC-01 to SEC-05 | 3, 5, 6 | Acceptance gates in every story touching network/storage |
| A11Y-01 to A11Y-05 | 1, 3, 4 | WCAG audit per epic completion |
| I18N-01 | 3 | Locale file + integration test |
| I18N-02 | 5 | Locale file set + integration test |
