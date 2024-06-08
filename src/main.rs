extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};
use clap::Parser;

/// Simple program to hash password using bcrypt
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value_t = DEFAULT_COST)]
    cost: u32,

    /// Password
    #[arg(required = false, default_value = "")]
    password: String,
}

fn main() {
    let args = Args::parse();
    let mut password = args.password;

    if password.len() == 0 {
        use std::io::{self, BufRead};

        let stdin = io::stdin();
        stdin.lock().read_line(&mut password).unwrap();
    }

    let hashed = hash(password.clone(), args.cost).unwrap();
    let valid = verify(password.clone(), &hashed).unwrap();

    if !valid {
        eprintln!("Invalid hash");
        std::process::exit(1);
    }

    println!("{:?}", hashed);
}
