use std::env;
use kvs::KvStore;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: kvs <COMMAND> [ARGS]");
        exit(1);
    }

    let mut store = KvStore::new();

    match args[1].as_str() {
        "set" => {
            if args.len() != 4 {
                eprintln!("Usage: kvs set <KEY> <VALUE>");
                exit(1);
            }
            store.set(args[2].clone(), args[3].clone());
        }
        "get" => {
            if args.len() != 3 {
                eprintln!("Usage: kvs get <KEY>");
                exit(1);
            }
            match store.get(args[2].clone()) {
                Some(value) => println!("{}", value),
                None => eprintln!("Key not found"),
            }
        }
        "rm" => {
            if args.len() != 3 {
                eprintln!("Usage: kvs rm <KEY>");
                exit(1);
            }
            match store.remove(args[2].clone()) {
                Some(_) => (),
                None => eprintln!("Key not found"),
            }
        }
        "-V" => {
            println!("kvs version {}", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            exit(1);
        }
    }
}
