// use std::process::ExitCode;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    match args.next().as_deref() {
        Some("--version") => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
        }
        Some("--help") => {
            println!(
                "Repeatedly output a line with all specified STRING(s), or 'y'.
                --help display this help and exit
                --version
                  output version information and exit"
            );
        }
        Some(arg) => {
            // let mut string: String = arg.to_string();
            let mut string: String = String::new();
            string.push_str(arg.to_string().as_str());
            if args.len() > 0 {
                for arg in args {
                    string.push_str(" ");
                    string.push_str(arg.as_str());
                }
            }
            loop {
                println!("{string}");
            }
        }
        None => {
            loop {
                println!("y");
                // return ExitCode::SUCCESS;
            }
        }
    }
}
