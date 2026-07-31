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
        Some(arg) => match convert_arg_to_duration(arg) {
            Ok(v) => {
                thread::sleep(Duration::from_secs(v));
            }
            Err(e) => {
                println!("{e}");
            }
        },
        None => {
            println!("sleep requires input,\ntry sleep --help");
        }
    }
}

fn convert_arg_to_duration(val: &str) -> Result<u64, String> {
    match val.trim().parse() {
        Ok(num) => Ok(num),
        Err(_err) => Err(format!("'{val}' interval can not be used").to_string()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn successful_input_conversion_test() {
        let output = super::convert_arg_to_duration("1");
        assert_eq!(output, Ok(1));
    }
    #[test]
    fn failed_input_conversion_test() {
        let output = super::convert_arg_to_duration("v");
        assert_eq!(output, Err("'v' interval can not be used".to_string()));
    }
}
