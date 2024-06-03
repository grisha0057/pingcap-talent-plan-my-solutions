use std::env;
use kvs::KvStore;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: kvs <COMMAND> [ARGS]");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "set" => {
            if args.len() != 4 {
                eprintln!("Usage: kvs set <KEY> <VALUE>");
                std::process::exit(1);
            }
            let mut store = KvStore::new();
            store.set(args[2].clone(), args[3].clone());
            println!("unimplemented");
        }
        "get" => {
            if args.len() != 3 {
                eprintln!("Usage: kvs get <KEY>");
                std::process::exit(1);
            }
            let store = KvStore::new();
            match store.get(args[2].clone()) {
                Some(value) => println!("{}", value),
                None => println!("unimplemented"),
            }
        }
        "rm" => {
            if args.len() != 3 {
                eprintln!("Usage: kvs rm <KEY>");
                std::process::exit(1);
            }
            let mut store = KvStore::new();
            store.remove(args[2].clone());
            println!("unimplemented");
        }
        "-V" => {
            println!("kvs version 0.1.0");
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}
