use comrade::{exit, lexer::Parser, read_file, write_file};
use open::that;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let raw_path = args.get(1);
    let print_tokens = args.contains(&"-t".to_string());
    let print_ast = args.contains(&"-a".to_string());
    let print_c_code = args.contains(&"-c".to_string());

    let out_path = "out.c";
    match raw_path {
        Some(path) => {
            if path == "report" {
                handle_open("https://github.com/FloatingComet62/comrade/issues/new?assignees=&labels=&template=bug_report.md&title=")
            }
            let data = read_file(&path);
            let parser = Parser::new(data);
            let (_program, c_code) = parser.parse(true, print_tokens, print_ast, print_c_code);
            if let Err(e) = write_file(out_path, c_code) {
                exit(
                    &format!("Failed to write to {}\nError Trace:\n{}", out_path, e),
                    None,
                )
            }
        }
        None => exit("No input files passed", None),
    }
}

fn handle_open(s: &str) {
    match that(s) {
        Ok(_) => exit("", Some(0)),
        Err(e) => exit(
            &format!("Failed to open the url\nLink: {}\nError Trace:\n{}", s, e),
            None,
        ),
    }
}
