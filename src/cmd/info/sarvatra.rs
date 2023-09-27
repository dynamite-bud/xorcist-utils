use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdSarvatra;

impl CliCommand for CmdSarvatra {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
