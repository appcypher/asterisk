//! Prompt Bar library

#![warn(missing_docs)]
#![allow(clippy::module_inception)]

mod app;
mod error;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use app::*;
pub use error::*;

pub(crate) mod cmd;
pub(crate) mod plugins;
pub(crate) mod window;
