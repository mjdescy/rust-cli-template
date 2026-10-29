use clap::{ArgAction, Parser, Subcommand};
use log::{LevelFilter, debug};

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

    let level = match cli.verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    // RUST_LOG overrides the level derived from -v.
    env_logger::Builder::new()
        .filter_level(level)
        .parse_default_env()
        .init();

    debug!("verbosity = {}", cli.verbose);

    match cli.command {
        Commands::Hello { name } => println!("Hello, {name}!"),
    }
}
