extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing password argument.");
        std::process::exit(1);
    }

    let hashed = hash(args[1].clone(), DEFAULT_COST).unwrap();
    let valid = verify(args[1].clone(), &hashed).unwrap();

    if !valid {
        eprintln!("Invalid hash");
        std::process::exit(1);
    }

    println!("{:?}", hashed);
}
