mod error;
mod messaging;
mod recorder;

use std::sync::{Mutex, OnceLock};

use oxichrome::log;

/// A global session handle set during `start()` and available for the
/// panic hook and message handlers.
static SESSION: OnceLock<Mutex<recorder::RecordingSession>> = OnceLock::new();

fn init_session() -> &'static Mutex<recorder::RecordingSession> {
    SESSION.get_or_init(|| Mutex::new(recorder::RecordingSession::new()))
}

#[oxichrome::extension(
    name = "Capture Forge",
    version = "0.1.0",
    permissions = ["storage"]
)]
struct Extension;

#[oxichrome::background]
async fn start() {
    // Install a custom panic hook that prevents WASM instance death.
    //
    // Without this, any Rust panic inside a WASM module would abort the
    // extension's entire WebAssembly instance, killing the service worker.
    std::panic::set_hook(Box::new(|panic_info| {
        let details = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "unknown panic cause".to_string()
        };

        let location = panic_info
            .location()
            .map(|loc| format!("{}:{}", loc.file(), loc.line()))
            .unwrap_or_else(|| "unknown location".into());

        let message = format!("{} — at {}", details, location);

        log!("[PANIC] {}", &message);

        // Attempt to transition the global session to Error state.
        //
        // If the session has not yet been initialised, the transition is
        // skipped — the extension will recover on next user interaction.
        if let Some(mutex) = SESSION.get() {
            if let Ok(mut session) = mutex.try_lock() {
                let _ = session.transition(recorder::SessionState::Error);
            }
        }

        // Re-register the default hook so the runtime still gets the panic
        // for diagnostic purposes, then re-invoke it.
        //
        // Note: set_hook replaces the current hook.  We deliberately do NOT
        // call abort or std::process::exit — the WASM instance survives.
        let _prev = std::panic::take_hook();
    }));

    log!("Capture Forge started!");

    // Initialise the global session so the panic hook can reference it.
    init_session();
}

#[oxichrome::on(runtime::on_installed)]
async fn handle_install(details: oxichrome::__private::wasm_bindgen::JsValue) {
    log!("Capture Forge installed: {:?}", details);
}
