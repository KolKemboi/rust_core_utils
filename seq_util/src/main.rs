use std::env;

fn main() {
    let mut args = env::args();
    args.next();

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

If FIRST or INCREMENT is omitted, it defaults to 1.  That is, an
omitted INCREMENT defaults to 1 even when LAST is smaller than FIRST.
The sequence of numbers ends when the sum of the current number and
INCREMENT would become greater than LAST.
FIRST, INCREMENT, and LAST are interpreted as floating point values.
INCREMENT is usually positive if FIRST is smaller than LAST, and
INCREMENT is usually negative if FIRST is greater than LAST.
INCREMENT must not be 0; none of FIRST, INCREMENT and LAST may be NaN.
FORMAT must be suitable for printing one argument of type 'double';
it defaults to %.PRECf if FIRST, INCREMENT, and LAST are all fixed point
decimal numbers with maximum precision PREC, and to %g otherwise.
        ",
    );

    match args.next().as_deref() {
        Some("--help") => {
            println!("{help_msg}");
        }
        Some("--version") => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        Some("-f") | Some("--format") => match args.next().as_deref() {
            Some("%f") => {
                let default = 1;

                let arg_length = args.len();

                if arg_length == 3 {
                    let start = match convert_to_int(args.next().as_deref().unwrap()) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };
                    let step_size = match convert_to_int(args.next().as_deref().unwrap()) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };
                    let end = match convert_to_int(args.next().as_deref().unwrap()) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };
                    for i in (start..=end).step_by(step_size as usize) {
                        println!("{:.4}", i as f64);
                    }
                } else if arg_length == 2 {
                    let start = match convert_to_int(args.next().as_deref().unwrap()) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };
                    let end = match convert_to_int(args.next().as_deref().unwrap()) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };
                    for i in (start..=end).step_by(1) {
                        println!("{:.4}", i as f64);
                    }
                } else if arg_length == 1 {
                    let end = match convert_to_int(args.next().as_deref().unwrap()) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };
                    for i in (default..=end).step_by(1) {
                        println!("{:.4}", i as f64);
                    }
                }
            }
            Some(_) => {}
            None => {}
        },
        Some("-s") | Some("--separator") => {
            let separator = args.next().unwrap();
            let default = 1;

            let arg_length = args.len();

            if arg_length == 3 {
                let start = match convert_to_int(args.next().as_deref().unwrap()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                let step_size = match convert_to_int(args.next().as_deref().unwrap()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                let end = match convert_to_int(args.next().as_deref().unwrap()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                for i in (start..=end).step_by(step_size as usize) {
                    print!("{}{}", i, &separator);
                }
            } else if arg_length == 2 {
                let start = match convert_to_int(args.next().as_deref().unwrap()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                let end = match convert_to_int(args.next().as_deref().unwrap()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                for i in (start..=end).step_by(1) {
                    print!("{}{}", i, &separator);
                }
            } else if arg_length == 1 {
                let end = match convert_to_int(args.next().as_deref().unwrap()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                for i in (default..=end).step_by(1) {
                    print!("{}{}", i, &separator);
                }
            }
        }

        Some("-w") | Some("--equal-width") => {
            for arg in args {
                let var = arg.len();
                println!("{}", &var);
                let val = convert_to_int(&arg.to_string());
                match val {
                    Ok(v) => println!("{}", v),
                    Err(e) => println!("{}", e),
                }
            }
        }

        Some(arg) => {
            let value_1 = match convert_to_int(arg) {
                Ok(v) => v,
                Err(e) => {
                    println!("{e}");
                    println!("Try 'seq --help' for more information.");
                    return;
                }
            };

            let arg_length = args.len();

            if arg_length == 2 {
                let step_size = match convert_to_int(args.next().as_deref().unwrap()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                let end = match convert_to_int(args.next().as_deref().unwrap()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                for i in (value_1..=end).step_by(step_size as usize) {
                    println!("{}", i);
                }
            } else if arg_length == 1 {
                let end = match convert_to_int(args.next().as_deref().unwrap()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                };
                for i in (value_1..=end).step_by(1) {
                    println!("{}", i);
                }
            } else {
                for i in (1..=value_1).step_by(1) {
                    println!("{}", i);
                }
            }
        }

        None => {
            let message: String = String::from(
                "
seq: missing operand
Try 'seq --help' for more information
                ",
            );
            println!("{message}");
        }
    }
}

fn convert_to_int(val: &str) -> Result<i64, String> {
    match val.trim().parse() {
        Ok(value) => Ok(value),
        Err(_err) => Err(format!("seq_util: invalid floating point argument: {val}")),
    }
}
