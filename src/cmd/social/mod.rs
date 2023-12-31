use super::CliCommand;

mod github;
mod instagram;
mod linkedin;
mod twitter;

#[derive(clap::Subcommand, Debug)]
/// This command shows you the info about my social profiles
pub enum CmdSocial {
    /// Info about my Github profile
    Github(github::CmdGithub),
    /// Info about my Linkedin profile
    Linkedin(linkedin::CmdLinkedin),
    /// Info about my Twitter profile
    Twitter(twitter::CmdTwitter),
    /// Info about my Instagram profile
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
