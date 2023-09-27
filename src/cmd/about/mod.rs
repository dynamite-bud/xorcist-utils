pub mod me;
pub mod website;

use clap::{Parser, Subcommand};

use super::CliCommand;

#[derive(Subcommand, Debug)]
pub enum CmdAbout {
    Me(me::CmdMe),
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
