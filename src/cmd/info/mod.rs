use super::CliCommand;

mod queezy;
mod ring;
mod sarvatra;
mod truck_navigation;
mod wasmer;

#[derive(clap::Subcommand, Debug)]
pub enum CmdInfo {
    Queezy(queezy::CmdQueezy),
    Ring(ring::CmdRing),
    Sarvatra(sarvatra::CmdSarvatra),
    TruckNavigation(truck_navigation::CmdTruckNavigation),
    Wasmer(wasmer::CmdWasmer),
}

impl CliCommand for CmdInfo {
    fn run(self) -> Result<(), anyhow::Error> {
        match self {
            Self::Queezy(cmd) => cmd.run(),
            Self::Ring(cmd) => cmd.run(),
            Self::Sarvatra(cmd) => cmd.run(),
            Self::TruckNavigation(cmd) => cmd.run(),
            Self::Wasmer(cmd) => cmd.run(),
        }
    }
}
