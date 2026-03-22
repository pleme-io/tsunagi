use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tsunagi", about = "macOS-Android bridge daemon")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the bridge daemon
    Daemon,
    /// Show bridge status
    Status,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Daemon => {
            println!("tsunagi: bridge daemon starting");
        }
        Command::Status => {
            println!("tsunagi: no active bridge connections");
        }
    }
}
