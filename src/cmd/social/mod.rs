use super::CliCommand;

mod github;
mod instagram;
mod linkedin;
mod twitter;

#[derive(clap::Subcommand, Debug)]
pub enum CmdSocial {
    Github(github::CmdGithub),
    Linkedin(linkedin::CmdLinkedin),
    Twitter(twitter::CmdTwitter),
    Instagram(instagram::CmdInstagram),
}

impl CliCommand for CmdSocial {
    fn run(self) -> Result<(), anyhow::Error> {
        match self {
            CmdSocial::Github(cmd) => cmd.run(),
            CmdSocial::Linkedin(cmd) => cmd.run(),
            CmdSocial::Twitter(cmd) => cmd.run(),
            CmdSocial::Instagram(cmd) => cmd.run(),
        }
    }
}
