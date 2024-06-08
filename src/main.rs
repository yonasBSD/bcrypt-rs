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
    #[arg(required=true)]
    password: String,
}

fn main() {
    let args = Args::parse();

    let hashed = hash(args.password.clone(), args.cost).unwrap();
    let valid = verify(args.password.clone(), &hashed).unwrap();

    if !valid {
        eprintln!("Invalid hash");
        std::process::exit(1);
    }

    println!("{:?}", hashed);
}
