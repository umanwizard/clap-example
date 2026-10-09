use clap::Parser;
use commands::Command;

/// Example CLI whose subcommands are defined in the `commands` crate
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Print the parsed arguments before running
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();
    if cli.verbose {
        eprintln!("{cli:?}");
    }
    println!("{}", cli.command.run());
}
