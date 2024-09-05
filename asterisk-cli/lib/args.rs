use clap::Parser;

use crate::styles;

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

#[derive(Debug, Parser)]
#[command(name = "asterisk", author, about, version, styles=styles::styles())]
pub struct AsteriskArgs {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    Serve {},
    Shell {},
}
