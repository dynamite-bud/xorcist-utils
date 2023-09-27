use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdInstagram;

impl CliCommand for CmdInstagram {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
