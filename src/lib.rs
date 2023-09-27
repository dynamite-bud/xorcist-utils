pub mod cmd;

use clap::Parser;
use cmd::CliCommand;

// macro_rules! print_typewriter {
//     ($text:expr,$color:expr, $delay:expr) => {
//         print!("\x1b[ q");
//         for c in $text.chars() {
//             print!("{}", c.to_string().color($color).bold());
//             io::stdout().flush().unwrap();
//             thread::sleep($delay);
//         }
//         print!("\x1b[ q");
//     };
// }

#[derive(Parser, Debug)]
#[clap(
    name = "x",
    version = "0.3.6",
    author = "xorcist",
    about = "A CLI tool for Xorcist's Peronsal Information. Use `--help` to get more information.",
    long_about = r#"
This is a CLI tool made in WebAssembly System Interface to navigate to different pages to get to know me, The Xorcist.
My moto: "Cleanse the code"

This tool is written in Rust and uses the Clap library for the CLI interface.
Okay enough of the boring stuff, let's get to know me.
    
> Just navigate around the different pages by using the subcommands.
> Use the --version flag to get the version of the tool.

Note: This tool is like a complete bash so you can probably do a lot more than you can imagine.
> P.S. You can even run python by just typing the command: `wasmer run python`
"#
)]
struct XCommands {
    /// Subcommands
    #[clap(subcommand)]
    cmd: cmd::SubCmd,
}

pub fn run() -> Result<(), anyhow::Error> {
    let x = XCommands::parse();
    x.cmd.run()
}
