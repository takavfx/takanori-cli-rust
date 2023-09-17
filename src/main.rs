mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Mkdd {
        #[arg(long = "path", required = false)]
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Mkdd { path } => {
            commands::make_today_dir(&path.to_string()).unwrap();
        }
    }
}
