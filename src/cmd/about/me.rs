use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdMe;

impl CliCommand for CmdMe {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
