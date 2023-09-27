use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdTruckNavigation;

impl CliCommand for CmdTruckNavigation {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
