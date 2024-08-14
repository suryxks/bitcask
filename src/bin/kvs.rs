use std::process;

use clap::{ Parser, Subcommand };

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None, author)]
struct Cli {
    /// get set and rm commands
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// Gets value associated with a kwy
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Rm {
        key: String,
    },
}
fn main() {
    let args = Cli::parse();
    loop {
        match args.command {
            Some(command) =>
                match command {
                    Commands::Get { key } => {
                        eprintln!("unimplemented");
                        process::exit(1);
                    }
                    Commands::Set { key, value } => {
                        eprintln!("unimplemented");
                        process::exit(1);
                    }
                    Commands::Rm { key } => {
                        eprintln!("unimplemented");
                        process::exit(1);
                    }
                }
            None => {
                process::exit(1);
            }
        }
    }
}
