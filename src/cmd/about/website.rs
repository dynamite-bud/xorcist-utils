use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdWebsite;

impl CliCommand for CmdWebsite {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
