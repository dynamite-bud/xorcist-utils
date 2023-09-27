use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdWasix;

impl CliCommand for CmdWasix {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
