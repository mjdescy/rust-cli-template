use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, arg_required_else_help = true)]
struct Cli {
    /// Increase output verbosity (repeat for more)
    #[arg(short, long, global = true, action = ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print a greeting
    Hello {
        /// Name to greet
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose > 0 {
        eprintln!("verbosity level: {}", cli.verbose);
    }

    match cli.command {
        Commands::Hello { name } => println!("Hello, {name}!"),
    }
}
