use std::env;
// use std::process::ExitCode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_count = args.len();
    // let mut default_start = 1;
    // let mut default_step = 1;
    let mut default_sep = "\n";

    let help_msg: String = String::from(
        "
Usage: seq [OPTION]... LAST
  or:  seq [OPTION]... FIRST LAST
  or:  seq [OPTION]... FIRST INCREMENT LAST
Print numbers from FIRST to LAST, in steps of INCREMENT.

Mandatory arguments to long options are mandatory for short options too.
  -f, --format=FORMAT      use printf style floating-point FORMAT
  -s, --separator=STRING   use STRING to separate numbers (default: \n)
  -w, --equal-width        equalize width by padding with leading zeroes
      --help        display this help and exit
      --version     output version information and exit
        ",
    );

    if args_count == 1 {
        println!("{}", help_msg);
        // return std::process::exit(1);
    }
    if args_count > 6 {
        println!("Too many arguments");
    }
    if args_count > 1 && args_count <= 6 {
        if args[1] == "--help" {
            println!("{}", help_msg);
        } else if args[1] == "--version" {
            println!("{}", env!("CARGO_PKG_VERSION"));
        } else if args[1] == "-s" {
            println!("Separator branch");
            if args_count <= 3 {
                println!("seq: failure");
                std::process::exit(1);
            }
            default_sep = &args[2];
            if args_count == 4 {
                print_one_out(&args[3].to_string(), default_sep.to_string(), false, false);
            }
            if args_count == 5 {
                print_two_out(
                    &args[3].to_string(),
                    &args[4].to_string(),
                    default_sep.to_string(),
                    false,
                    false,
                );
            }
            if args_count == 6 {
                print_three_out(
                    &args[3].to_string(),
                    &args[4].to_string(),
                    &args[5].to_string(),
                    default_sep.to_string(),
                    false,
                    false,
                );
            }
        } else if args[1] == "-w" {
            println!("Wide Branch");

            if args_count <= 3 || args_count > 5 {
                println!("seq: failure");
                std::process::exit(1);
            }

            if args_count == 3 {
                print_one_out(&args[2].to_string(), default_sep.to_string(), true, false);
            }
            if args_count == 4 {
                print_two_out(
                    &args[2].to_string(),
                    &args[3].to_string(),
                    default_sep.to_string(),
                    true,
                    false,
                );
            }
            if args_count == 5 {
                print_three_out(
                    &args[2].to_string(),
                    &args[3].to_string(),
                    &args[4].to_string(),
                    default_sep.to_string(),
                    true,
                    false,
                );
            }
        } else if args[1] == "-f" {
            println!("Formatted Branch");
            if args_count <= 3 {
                println!("seq: failure");
                std::process::exit(1);
            }
            if args[2] != "%f" {
                println!("%f is required");
                std::process::exit(1);
            }
            if args_count == 4 {
                print_one_out(&args[3].to_string(), default_sep.to_string(), false, true);
            }
            if args_count == 5 {
                print_two_out(
                    &args[3].to_string(),
                    &args[4].to_string(),
                    default_sep.to_string(),
                    false,
                    true,
                );
            }
            if args_count == 6 {
                print_three_out(
                    &args[3].to_string(),
                    &args[4].to_string(),
                    &args[5].to_string(),
                    default_sep.to_string(),
                    false,
                    true,
                );
            }
        } else {
            println!("Final Branch");
            if args_count > 4 {
                println!("too much args");
                std::process::exit(1);
            }
            if args_count == 2 {
                print_one_out(&args[1].to_string(), default_sep.to_string(), false, false);
            }
            if args_count == 3 {
                print_two_out(
                    &args[1].to_string(),
                    &args[2].to_string(),
                    default_sep.to_string(),
                    false,
                    false,
                );
            }
            if args_count == 4 {
                print_three_out(
                    &args[1].to_string(),
                    &args[2].to_string(),
                    &args[3].to_string(),
                    default_sep.to_string(),
                    false,
                    false,
                );
            }
        }
    }
}
fn print_one_out(arg: &String, sep: String, wide: bool, is_float: bool) {
    let mut end: i64 = 1;
    match convert_to_int(&arg) {
        Ok(v) => end = v,
        Err(e) => println!("{}", e),
    }
    let width = end.to_string().len();

    if wide {
        for i in 1..=end {
            print!("{:0width$}{}", i, sep, width = width)
        }
    } else if is_float {
        for i in 1..=end {
            print!("{:.4}{}", i as f64, sep)
        }
    } else {
        for i in 1..=end {
            print!("{}{}", i, sep)
        }
    }
}
fn print_two_out(start_arg: &String, end_arg: &String, sep: String, wide: bool, is_float: bool) {
    let mut end: i64 = 1;
    let mut start: i64 = 1;
    match convert_to_int(&start_arg) {
        Ok(v) => start = v,
        Err(e) => println!("{}", e),
    }
    match convert_to_int(&end_arg) {
        Ok(v) => end = v,
        Err(e) => println!("{}", e),
    }
    let width = end.to_string().len();

    if wide {
        for i in start..=end {
            print!("{:0width$}{}", i, sep, width = width)
        }
    } else if is_float {
        for i in start..=end {
            print!("{:.4}{}", i as f64, sep)
        }
    } else {
        for i in start..=end {
            print!("{}{}", i, sep)
        }
    }
}
fn print_three_out(
    start_arg: &String,
    step_arg: &String,
    end_arg: &String,
    sep: String,
    wide: bool,
    is_float: bool,
) {
    let mut end: i64 = 1;
    let mut step: i64 = 1;
    let mut start: i64 = 1;
    match convert_to_int(&start_arg) {
        Ok(v) => start = v,
        Err(e) => println!("{}", e),
    }
    match convert_to_int(&end_arg) {
        Ok(v) => end = v,
        Err(e) => println!("{}", e),
    }
    match convert_to_int(&step_arg) {
        Ok(v) => step = v,
        Err(e) => println!("{}", e),
    }
    let width = end.to_string().len();

    if wide {
        for i in (start..=end).step_by(step as usize) {
            print!("{:0width$}{}", i, sep, width = width)
        }
    } else if is_float {
        for i in (start..=end).step_by(step as usize) {
            print!("{:.4}{}", i as f64, sep)
        }
    } else {
        for i in (start..=end).step_by(step as usize) {
            print!("{}{}", i, sep)
        }
    }
}
fn convert_to_int(val: &str) -> Result<i64, String> {
    match val.trim().parse() {
        Ok(value) => Ok(value),
        Err(_err) => Err(format!("seq_util: invalid floating point argument: {val}")),
    }
}
