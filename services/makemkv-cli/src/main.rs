use clap::{Parser, Subcommand};

mod commands;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Devices(commands::Devices),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Devices(args) => commands::device_execution(args),
    }
}
