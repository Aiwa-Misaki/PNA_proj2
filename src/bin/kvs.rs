use clap::{Parser, Subcommand};
use kvs::KvStore;
use std::env;

#[derive(Parser)]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

fn main() {
    let cli = Cli::parse();
    let dir = env::current_dir().unwrap();
    let mut kv = KvStore::open(dir.as_path()).unwrap();

    match &cli.command {
        Commands::Set { key, value } => {
            kv.set(key.clone(), value.clone())
                .map_err(|e| println!("{}", e))
                .unwrap();
        }
        Commands::Get { key } => {
            let result = kv.get(key.clone()).unwrap();
            if result.is_none() {
                println!("Key not found");
            } else {
                println!("{}", result.unwrap());
            }
        }
        Commands::Rm { key } => {
            kv.remove(key.clone())
                .map_err(|e| println!("{}", e))
                .unwrap();
        }
    }
}
