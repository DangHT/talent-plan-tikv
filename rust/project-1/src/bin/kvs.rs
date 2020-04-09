use clap::{App, Arg, SubCommand};
use kvs::store::KvStore;
use std::process;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set a KEY VALUE pair.")
                .arg(Arg::with_name("KEY"))
                .arg(Arg::with_name("VALUE")),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get VALUE by KEY.")
                .arg(Arg::with_name("KEY")),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a K/V pair by KEY")
                .arg(Arg::with_name("KEY")),
        )
        .get_matches();

    let mut stores = KvStore::new();

    match matches.subcommand() {
        ("set", Some(kv)) => {
            stores
                .set(
                    kv.value_of("KEY").unwrap().to_string(),
                    kv.value_of("VALUE").unwrap().to_string(),
                )
                .unwrap_or_else(|| {
                    eprintln!("unimplemented");
                    process::exit(1);
                });
        }
        ("get", Some(key)) => {
            let v = stores
                .get(key.value_of("KEY").unwrap().to_string())
                .unwrap_or_else(|| {
                    eprintln!("unimplemented");
                    process::exit(1);
                });
            println!("{}", v);
        }
        ("rm", Some(key)) => {
            stores
                .remove(key.value_of("KEY").unwrap().to_string())
                .unwrap_or_else(|| {
                    eprintln!("unimplemented");
                    process::exit(1);
                });
        }
        (&_, _) => {
            eprintln!("unimplemented");
            process::exit(1);
        }
    }
}
