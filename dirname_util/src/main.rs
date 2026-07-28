use std::path::Path;
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args();
    args.next();

    // let _version: String = String::from("-v");

    match args.next().as_deref() {
        Some("-v") | Some("--version") => {
            println!("Version 0.1.0");
        }
        Some("-h") | Some("--help") => {
            println!(
                "dirname /usr/bin/
              -> '/usr'

       dirname dir1/str dir2/str
              -> 'dir1' followed by 'dir2'

       dirname stdio.h
              -> '.'"
            );
        }
        Some("/") => {
            println!("/");
        }
        Some("-z") | Some("--zero") => {
            for arg in args {
                let dirname = process_path(arg.as_str());
                print!("{dirname}");
            }
        }
        Some(filepath) => {
            let dirname = process_path(filepath);
            println!("{dirname}");
            for arg in args {
                let dirname = process_path(arg.as_str());
                println!("{dirname}");
            }
        }
        None => {
            eprintln!("Error:: Enter atleast one arguement");
            std::process::exit(1);
        }
    }

    ExitCode::SUCCESS
}

fn process_path(filepath: &str) -> String {
    if filepath.contains("/") == false {
        return String::from(".");
    } else {
        let dir = Path::new(filepath).parent().unwrap().to_string_lossy();
        return String::from(dir);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn process_path_test() {
        assert_eq!(super::process_path("home/kol/bin"), "home/kol");
    }
}
