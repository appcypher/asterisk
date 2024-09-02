// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use asterisk::{Asterisk, Result};

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() -> Result<()> {
    Asterisk::new()?.run();
    Ok(())
}
