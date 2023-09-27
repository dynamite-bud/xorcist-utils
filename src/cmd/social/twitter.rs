use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdTwitter;

impl CliCommand for CmdTwitter {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
