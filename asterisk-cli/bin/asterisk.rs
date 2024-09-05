use asterisk::{shell, AsteriskArgs, CliResult, Subcommand};
use clap::{CommandFactory, Parser};

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

#[tokio::main]
async fn main() -> CliResult<()> {
    // Parse command line arguments
    match AsteriskArgs::parse().subcommand {
        Some(Subcommand::Serve {}) => {
            println!("Coming soon...");
        }
        Some(Subcommand::Shell {}) => shell::run().await?,
        None => AsteriskArgs::command().print_help()?,
    }

    Ok(())
}
