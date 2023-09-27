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
struct XCommands {
    #[clap(subcommand)]
    cmd: cmd::SubCmd,
}

pub fn run() -> Result<(), anyhow::Error> {
    let x = XCommands::parse();
    x.cmd.run()
}
