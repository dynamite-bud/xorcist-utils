pub mod me;
pub mod website;

use clap::{Parser, Subcommand};

use super::CliCommand;

#[derive(Subcommand, Debug)]
/// This command shows you the info about me and this website
pub enum CmdAbout {
    /// Show info about me
    Me(me::CmdMe),
    /// Show info about this website
    Website(website::CmdWebsite),
}

impl CliCommand for CmdAbout {
    fn run(self) -> Result<(), anyhow::Error> {
        match self {
            Self::Me(cmd) => cmd.run(),
            Self::Website(cmd) => cmd.run(),
        }
    }
}
