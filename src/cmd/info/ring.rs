use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdRing;

impl CliCommand for CmdRing {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
