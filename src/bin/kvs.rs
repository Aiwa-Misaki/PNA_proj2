use clap::{Parser, Subcommand};
use kvs::KvStore;

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
    let mut kv = KvStore::new();

    match &cli.command {
        Commands::Set { key, value } => {
            println!("'kvs set' was used, {key}:{value}");
            kv.set(key.clone(), value.clone());
        }
        Commands::Get { key } => {
            println!("'kvs get' was used, {key}");
            kv.get(key.clone());
        }
        Commands::Rm { key } => {
            println!("'kvs rm' was used, {key}");
            kv.remove(key.clone());
        }
    }
}
