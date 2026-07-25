use std::env;

fn echo(args: &Vec<String>) -> String {
    args.join(" ")
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    println!("{}", echo(&args));
}

#[cfg(test)]
mod tests {
    #[test]
    fn join_multiple_arguments() {
        let args = vec!["hello".to_string(), "world".to_string()];

        assert_eq!(super::echo(&args), "hello world");
    }

    #[test]
    fn join_empty_arguments() {
        let args: Vec<String> = Vec::new();

        assert_eq!(super::echo(&args), "");
    }
}
