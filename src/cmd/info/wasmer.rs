use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdWasmer;

impl CliCommand for CmdWasmer {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
