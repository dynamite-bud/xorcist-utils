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

        // My contact info
        message.push(format!(
            "\r\n{}\r\n",
            "📞 Contact Info 📧".bright_purple().bold().underline()
        ));

        message.push(format!(
            "Email: {}",
            "ru.tcd.irl@gmail.com".bright_blue().underline(),
        ));

        message.push(format!("\r\n💁: This page is more about my personal likings. If you'd like to know more about my projects use the command `{}`","x info".underline().bold()));

        // My favourite Music genres and albums
        message.push(format!(
            "\r\n{}\r\n",
            "♯ Music ♫".bright_blue().bold().underline()
        ));

        message.push(format!(
            "I love to play the Guitar 🎸. I play both {} and {}",
            "acoustic".bright_white().underline(),
            "fingerstyle".bright_white().underline()
        ));

        message.push(format!(
            "I listen to {}, {} and {}",
            "DHH".bright_red().bold(),
            "Rap".bright_blue().bold(),
            "Rock".bright_black().bold(),
        ));

        message.push(
            "\r\nSome of my favourite albums are:"
                .underline()
                .to_string(),
        );
        message.push(format!(
            r#"
▶ {} by {}
▶ {} by {}
▶ {} by {}
▶ {} by {}
▶ {} by {}
"#,
            "Bayaan".bright_yellow().bold(),
            "Seedhe Maut".bright_red().bold(),
            "Nayaab".bright_yellow().bold(),
            "Seedhe Maut".bright_red().bold(),
            "Dark Side of the Moon".bright_blue().bold(),
            "Pink Floyd".bright_black().bold(),
            "The Unforgiven".magenta().bold(),
            "Metallica".bright_black().bold(),
            "Twenty One Pilots".bright_green().bold(),
            "Twenty One Pilots".bright_cyan().bold(),
        ));

        // My favourite Movie genres and movies
        message.push(format!(
            "\r\n{}\r\n",
            "🎬 Movies 🎥".bright_purple().bold().underline()
        ));

        message.push(format!(
            "I like to watch {}, {} and {}",
            "Drama".bright_red().bold(),
            "Psycological Thrillers".bright_blue().bold(),
            "Bollywood".bright_black().bold(),
        ));

        message.push("\r\nSome of my favourite movies:".underline().to_string());
        message.push(format!(
            r#"
▶ {}
▶ {}
▶ {}
▶ {}
▶ {}
"#,
            "American Beauty".purple().bold(),
            "Silence of the Lambs".yellow().bold(),
            "Interstellar ✨".bright_black().bold(),
            "Drishyam".bright_red().bold(),
            "Shivaji: The Boss".bright_blue().bold(),
        ));

        // My favourite TV shows
        message.push(format!(
            "\r\n{}\r\n",
            "📺 TV Shows 📺".bright_purple().bold().underline()
        ));

        message.push(format!(
            "I like to watch {}, {} and {}",
            "Sitcom".bright_red().bold(),
            "Cartoons".bright_blue().bold(),
            "Science Shows".bright_black().bold(),
        ));

        message.push("\r\nSome of my favourite TV shows:".underline().to_string());
        message.push(format!(
            r#"
▶ {}
▶ {}
▶ {}
▶ {}
▶ {}
▶ {}
"#,
            "South Park".bright_white().bold(),
            "Breaking Bad".bright_green().bold(),
            "Better Call Saul".bright_red().bold(),
            "Curb Your Enthusiasm".bright_yellow().bold(),
            "Raja, Rasoi aur Anya Kahaniyan".bright_red().bold(),
            "Chef's Table".bright_purple().bold(),
        ));

        let message = message.join("\r\n");
        println!("{}", message);
        Ok(())
    }
}
