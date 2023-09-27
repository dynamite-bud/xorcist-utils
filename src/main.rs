pub mod cmd;

use colored::{Color, Colorize};
use std::{error::Error, time::Duration};

enum XCommands {
    Info,
    Social,
    About,
    Help,
}

impl From<&str> for XCommands {
    fn from(s: &str) -> Self {
        match s {
            "info" => Self::Info,
            "social" => Self::Social,
            "about" => Self::About,
            "help" => Self::Help,
            _ => {
                eprintln!("{}: {}", "Unknown command".bright_red(), s);
                std::process::exit(1);
            }
        }
    }
}

struct XRunner();

impl XRunner {
    fn new() -> Self {
        Self()
    }
    fn run(&self, command: XCommands) -> Result<(), Box<dyn Error>> {
        match command {
            XCommands::Info => self.info(),
            XCommands::Social => self.social(),
            XCommands::About => self.about(),
            XCommands::Help => self.help(),
        }
    }

    fn info(&self) -> Result<(), Box<dyn Error>> {
        println!("{}: {}", "Name".bright_green(), "X");
        println!("{}: {}", "Version".bright_green(), "0.1.0");
        println!("{}: {}", "Author".bright_green(), "X");
        println!("{}: {}", "License".bright_green(), "MIT");
        Ok(())
    }

    fn social(&self) -> Result<(), Box<dyn Error>> {
        println!("{}: {}", "Name".bright_green(), "X");
        println!("{}: {}", "Version".bright_green(), "0.1.0");
        println!("{}: {}", "Author".bright_green(), "X");
        println!("{}: {}", "License".bright_green(), "MIT");
        Ok(())
    }

    fn about(&self) -> Result<(), Box<dyn Error>> {
        println!("{}: {}", "Name".bright_green(), "X");
        println!("{}: {}", "Version".bright_green(), "0.1.0");
        println!("{}: {}", "Author".bright_green(), "X");
        println!("{}: {}", "License".bright_green(), "MIT");
        Ok(())
    }

    fn help(&self) -> Result<(), Box<dyn Error>> {
        let mut help_text = String::new();
        // print the usage information
        help_text.push_str(
            format!(
                "{}:\t {}",
                "Usage".bright_blue(),
                "x <command>".bright_white().bold()
            )
            .as_str(),
        );
        help_text.push_str("\r\n\r\n");
        // print the available commands
        help_text.push_str("Available Commands:\n\n".underline().to_string().as_str());
        help_text.push_str(
            format!(
                "▶ {}:\t\t{}",
                "info".bright_green().bold(),
                "Prints information about the available functionality"
            )
            .as_str(),
        );
        help_text.push_str("\r\n");
        help_text.push_str(
            format!(
                "▶ {}:\t{}",
                "about".cyan().bold(),
                "About the author of this program"
            )
            .as_str(),
        );
        help_text.push_str("\r\n");
        help_text.push_str(
            format!(
                "▶ {}:\t{}",
                "social".bright_magenta().bold(),
                "Prints the social media links of the author"
            )
            .as_str(),
        );
        help_text.push_str("\r\n");
        help_text.push_str(
            format!(
                "▶ {}:\t\t{}",
                "help".yellow().bold(),
                "Prints this help information"
            )
            .as_str(),
        );
        help_text.push_str("\r\n");
        println!("{}", help_text);
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let message = "Hello World";
    let color = Color::BrightBlack;
    let delay = Duration::from_millis(100);

    // print_typewriter!(message, color, delay);

    let runner = XRunner::new();

    // get command line arguments
    let args = std::env::args().collect::<Vec<_>>();

    // if only one argument is provided, print help
    if args.len() == 1 {
        // runner.run(XCommands::Help)?;
        return Ok(());
    }

    // let command = *input
    //     .first()
    //     .ok_or("No command provided, Please see help")?;

    // // run command
    // runner.run(XCommands::from(command))?;
    Ok(())
}
