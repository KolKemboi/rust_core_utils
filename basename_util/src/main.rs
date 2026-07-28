use std::path::Path;
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args();

    args.next(); // skips the exec name

    // make sure there is an input
    if args.len() == 0 {
        eprintln!("Error:: Atleast one argument required");
        std::process::exit(1);
    }

    // let name = Path::new(&path).file_name().unwrap().to_string_lossy();
    match args.next().as_deref() {
        Some("-v") | Some("--version") => {
            println!("Basename version 0.1.0");
        }
        Some("-h") | Some("--help") => {
            println!(
                "EXAMPLES
       basename /usr/bin/sort
              -> 'sort'

       basename include/stdio.h .h
              -> 'stdio'

       basename -s .h include/stdio.h
              -> 'stdio'

       basename -a any/str1 any/str2
              -> 'str1' followed by 'str2'"
            );
        }

        Some("/") => {
            println!("/");
        }
        //allows multiple input and outputs the basename
        Some("-a") => {
            for arg in args {
                let name = Path::new(arg.as_str())
                    .file_name()
                    .unwrap()
                    .to_string_lossy();
                println!("{name}")
            }
        }

        //prints without a newline character
        Some("-z") => match args.next() {
            Some(p) => {
                let name = Path::new(p.as_str()).file_name().unwrap().to_string_lossy();
                print!("{name}");
            }
            None => {}
        },

        //for flag s with or without ext
        Some("-s") => match args.next() {
            Some(extension) => {
                if extension.starts_with(".") {
                    match args.next() {
                        Some(p) => {
                            let name = Path::new(p.as_str()).file_name().unwrap().to_string_lossy();
                            if name.ends_with(&extension) {
                                println!("{}", name.strip_suffix(&extension).unwrap());
                            } else {
                                println!("{name}");
                            }
                        }
                        None => {}
                    }
                } else {
                    let name = Path::new(extension.as_str())
                        .file_name()
                        .unwrap()
                        .to_string_lossy();
                    println!("{name}");
                }
            }
            None => {}
        },

        // pure filename with possible extension
        Some(filename) => match args.next() {
            Some(extension) => {
                if extension.starts_with(".") {
                    let name = Path::new(filename).file_name().unwrap().to_string_lossy();
                    if name.ends_with(&extension) {
                        println!("{}", name.strip_suffix(&extension).unwrap());
                    } else {
                        println!("{name}");
                    }
                }
            }
            None => {
                let name = Path::new(filename).file_name().unwrap().to_string_lossy();
                println!("{name}")
            }
        },

        _ => {
            println!("Error");
        }
    }

    ExitCode::SUCCESS
}
