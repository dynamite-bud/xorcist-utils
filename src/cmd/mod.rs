pub mod about;
pub mod info;
pub mod social;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SubCmd {
    #[clap(subcommand)]
    Info(info::CmdInfo),
    #[clap(subcommand)]
    Social(social::CmdSocial),
    #[clap(subcommand)]
    About(about::CmdAbout),
}

impl CliCommand for SubCmd {
    fn run(self) -> Result<(), anyhow::Error> {
        match self {
            SubCmd::Info(cmd) => cmd.run(),
            SubCmd::Social(cmd) => cmd.run(),
            SubCmd::About(cmd) => cmd.run(),
        }
    }
}
pub trait CliCommand {
    fn run(self) -> Result<(), anyhow::Error>;
}
