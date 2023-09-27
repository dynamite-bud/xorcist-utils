use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdGithub;

impl CliCommand for CmdGithub {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
