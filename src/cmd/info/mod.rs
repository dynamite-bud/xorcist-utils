use super::CliCommand;

mod queezy;
mod ring;
mod sarvatra;
mod truck_navigation;
mod wasix;
mod wasmer;

#[derive(clap::Subcommand, Debug)]
/// This command shows you the info about my projects
pub enum CmdInfo {
    /// Wasmer - Pioneers of WebAssembly
    ///
    /// My contributions to the Wasmer Ecosystem
    Wasmer(wasmer::CmdWasmer),
    /// WASIX - The superset of WASI
    ///
    /// My contributions to the WASIX
    Wasix(wasix::CmdWasix),
    /// Ring - Crypto Library
    ///
    /// A crypto library I compiled to WASIX.
    Ring(ring::CmdRing),
    /// Truck Navigation
    ///
    /// A 3D navigation system for Mining trucks to be used in open cast mines.
    TruckNavigation(truck_navigation::CmdTruckNavigation),
    /// Queezy - Live Formative Assessment in Google Meet
    ///
    /// A chrome extension for teachers to take live formative assessments in a google meet.
    Queezy(queezy::CmdQueezy),
    /// Sarvatra - The Eye
    ///
    /// A hot swappable 3D printed camera with Multiple Sensors for Drones
    Sarvatra(sarvatra::CmdSarvatra),
}

impl CliCommand for CmdInfo {
    fn run(self) -> Result<(), anyhow::Error> {
        match self {
            Self::Queezy(cmd) => cmd.run(),
            Self::Ring(cmd) => cmd.run(),
            Self::Sarvatra(cmd) => cmd.run(),
            Self::TruckNavigation(cmd) => cmd.run(),
            Self::Wasmer(cmd) => cmd.run(),
            Self::Wasix(cmd) => cmd.run(),
        }
    }
}
