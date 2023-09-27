use crate::cmd::CliCommand;

#[derive(clap::Parser, Debug)]
pub struct CmdQueezy;

impl CliCommand for CmdQueezy {
    fn run(self) -> Result<(), anyhow::Error> {
        println!("Cmd me!");
        Ok(())
    }
}
