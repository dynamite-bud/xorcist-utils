use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdLinkedin;

impl CliCommand for CmdLinkedin {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
