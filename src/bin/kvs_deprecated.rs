use clap::{arg, command, Arg, ArgAction, Command};
use kvs::KvStore;

fn main() {
    let mut kv = KvStore::new();
    let matches = command!() // requires `cargo` feature
        .disable_version_flag(true)
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .help("Print version info")
                .action(ArgAction::SetTrue),
        )
        .subcommand_required(false)
        .subcommand(
            Command::new("set")
                .about("set key value")
                .arg(arg!([KEY]))
                .arg(arg!([VALUE])),
        )
        .subcommand(Command::new("get").about("get value").arg(arg!([KEY])))
        .subcommand(Command::new("rm").about("rm value").arg(arg!([KEY])))
        .get_matches();

    if matches.get_flag("version") {
        println!("kvs {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    match matches.subcommand() {
        Some(("set", sub_matches)) => {
            eprintln!("unimplemented");
            panic!();
            let key = sub_matches.get_one::<String>("KEY").unwrap().clone();
            let value = sub_matches.get_one::<String>("VALUE").unwrap().clone();
            kv.set(key, value);
        }
        Some(("get", sub_matches)) => {
            eprintln!("unimplemented");
            panic!();
            let key = sub_matches.get_one::<String>("KEY").unwrap().clone();
            kv.get(key);
        }
        Some(("rm", sub_matches)) => {
            eprintln!("unimplemented");
            panic!();
            let key = sub_matches.get_one::<String>("KEY").unwrap().clone();
            kv.remove(key);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
