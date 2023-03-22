use std::env;
use std::fs;

use comrade::{exit, parser::Parser};

fn main() {
    let _args: Vec<_> = env::args().collect();
    let raw_path = _args.get(1);
    match raw_path {
        Some(path) => {
            let data = read_file(&path);
            let parser = Parser::new(data);
            println!("{:?}", parser.parse());
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
