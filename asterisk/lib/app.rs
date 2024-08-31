//! The main entry point for the Prompt Bar

use tauri::{App, Manager, RunEvent};

use crate::{error::Result, plugins, window};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The main application struct.
pub struct Asterisk {
    /// The Tauri app instance.
    app: App,
}

/// The state of the Prompt Bar.
#[derive(Default)]
pub struct AppState {}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Asterisk {
    /// Creates a new instance of the Prompt Bar.
    pub fn new() -> Result<Self> {
        let builder = tauri::Builder::default();

        // Set invoke handlers
        let builder = builder.invoke_handler(tauri::generate_handler![]);

        // Setup
        let builder = builder.setup(|app| {
            // Initialize the app state
            app.manage(AppState::default());

            // Set up the plugins and window
            plugins::setup(app)?;
            window::setup(app)?;

            Ok(())
        });

        // Build app
        let app = builder.build(tauri::generate_context!("./tauri.conf.json"))?;

        Ok(Self { app })
    }

    /// Runs the Prompt Bar.
    pub fn run(self) {
        self.app.run(move |_app_handle, _event| {
            // Prevent the app from exiting.
            #[cfg(desktop)]
            if let RunEvent::ExitRequested { api, .. } = &_event {
                api.prevent_exit();
            }
        })
    }
}
