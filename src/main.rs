use std::env;
use std::fs;

use comrade::{exit, parser::Parser};

fn main() {
  let args: Vec<_> = env::args().collect();
  let raw_path = args.get(1);
  match raw_path {
    Some(path) => {
      let data = read_file(path);
      let parser = Parser::new(data);
      println!("{:?}", parser.program)
    }
    None => exit("No input files passes", None)
  }
  println!("Hello, world!");
}

fn read_file(path: &String) -> String {
  let contents = fs::read_to_string(path);
  match contents {
    Ok(data) => data,
    Err(e) => exit(&format!("Unable to read {}\nError Trace:\n{}", path, e), None)
  }
}