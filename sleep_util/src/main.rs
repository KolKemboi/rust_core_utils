use std::env;
use std::{thread, time::Duration};

fn main() {
    let mut args = env::args();
    let help_msg = "Usage: sleep NUMBER[SUFFIX]...\nor:  sleep OPTION \nPause for NUMBER seconds.  SUFFIX may be 's' for seconds (the default),\n'm' for minutes, 'h' for hours or 'd' for days.  NUMBER need not be an\ninteger.  Given two or more arguments, pause for the amount of time\nspecified by the sum of their values.\n\n\t--help        display this help and exit\n\t--version     output version information and exit";

    args.next();

    match args.next().as_deref() {
        Some("--help") => {
            println!("{}", help_msg);
        }
        Some("--version") => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        Some(arg) => {
            let secs: u64 = arg.trim().parse().expect("try entering a number suzan");
            thread::sleep(Duration::from_secs(secs));
        }
        None => {
            println!("sleep requires input,\ntry sleep --help");
        }
    }
}

fn convert_arg_to_duration(val: &str) -> Result<u64, String> {
    match val.trim().parse() {
        Ok(num) => Ok(num),
        Err(val) => Err("{val} can not be use".to_string()),
    }
}
