use std::env;
use std::fs;

use comrade::{exit, lexer::Parser};

fn main() {
    let args: Vec<_> = env::args().collect();
    let raw_path = args.get(1);
    let print_tokens = args.contains(&"-t".to_string());
    let print_ast = args.contains(&"-a".to_string());
    match raw_path {
        Some(path) => {
            let data = read_file(&path);
            let parser = Parser::new(data);
            parser.parse(print_tokens, print_ast);
        }
        None => exit("No input files passes", None),
    }
}

fn read_file(path: &String) -> String {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(data) => data,
        Err(e) => exit(
            &format!("Unable to read {}\nError Trace:\n{}", path, e),
            None,
        ),
    }
}
