use clap::Subcommand;

/// Subcommands understood by the CLI. Each variant's fields become that
/// subcommand's arguments.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print a greeting
    Greet {
        /// Who to greet
        name: String,

        /// Greet in all caps
        #[arg(short, long)]
        shout: bool,
    },
    /// Add two integers
    Add { a: i64, b: i64 },
}

impl Command {
    pub fn run(&self) -> String {
        match self {
            Command::Greet { name, shout } => {
                let greeting = format!("Hello, {name}!");
                if *shout {
                    greeting.to_uppercase()
                } else {
                    greeting
                }
            }
            Command::Add { a, b } => (a + b).to_string(),
        }
    }
}
