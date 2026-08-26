use std::fs;
use std::{env, process::exit};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // args.pop(); -- I am dumb, this removes the lasts val
    args.remove(0);
    println!("{}", &args.join(","));

    if args.len() == 0 {
        println!("ERROR::try cat --help");
        exit(1);
    }

    // make sure to leave after out
    if args[0] == "--help" {
        println!("HELP HELP");
        return;
    }
    if args[0] == "--version" {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    readfile(&args[0].to_string());
}

fn readfile(filename: &str) {
    let contents = fs::read_to_string(filename).expect("No file");

    println!("{contents}");
}
