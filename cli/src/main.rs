use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "obscura")]
#[command(about = "Obscura blockchain CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a full node
    Node,
    /// Start the miner
    Miner,
    /// Wallet operations
    Wallet,
}

fn main() {
    let _cli = Cli::parse();
    // TODO: dispatch to sub-modules
    println!("Obscura CLI stub");
}
