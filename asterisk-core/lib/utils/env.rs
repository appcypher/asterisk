use std::path::PathBuf;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The environment to load.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Env {
    /// The development environment.
    Dev,

    /// The production environment.
    Prod,
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Load the environment variables from the given environment.
pub fn load_env(r#type: Env) -> Option<PathBuf> {
    let env = match r#type {
        Env::Dev => ".env.dev",
        Env::Prod => ".env",
    };

    dotenvy::from_filename(env).ok()
}
