use crate::cmd::CliCommand;

use colored::Colorize;
#[derive(clap::Parser, Debug)]
/// This command shows you the info about me
pub struct CmdMe;

impl CliCommand for CmdMe {
    fn run(self) -> Result<(), anyhow::Error> {
        let mut message = Vec::<String>::new();
        message.push(format!("{} 👋\r\n", "Hi".bright_black().bold()));
        message.push(format!(
            "My name is {}, I am a Software Engineer + DevRel at Wasmer.io.",
            "Rudra".bright_white().bold().underline()
        ));
        message.push(format!("I ❤️ {}", "Open Source".bright_green().bold()));
        message.push(
            format!("I am a {} 🦀, I love to code in {} but I have also dabbled quite a lot \nwith {}/{} and Web Development in {}, {} with {}.",
                    "Rustacean".bright_red().bold(),
                    "Rust".red().bold(),
                    "JavaScript".bright_yellow().bold(),
                    "TypeScript".bright_blue().bold(),
                    "⚛ React".bright_cyan().bold(),
                    "Next.js".bright_black().bold(),
                    "Recoil".magenta().bold())
                );

        message.push(
            format!("I am a {} enthusiast, I love to work with WASM/WASI/WASIX and often contribute to the ecosystem.","WebAssembly".bright_blue().bold())
        );

        message.push(format!(
            "\r\n{}\r\n",
            "♯ Music ♫".bright_blue().bold().underline()
        ));
        message.push(format!(
            "I love to play the Guitar 🎸. I play both {} and {}",
            "acoustic".bright_white().underline(),
            "fingerstyle".bright_white().underline()
        ));

        let message = message.join("\r\n");
        println!("{}", message);
        Ok(())
    }
}
