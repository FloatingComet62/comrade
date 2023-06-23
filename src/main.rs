use comrade::{exit, lexer::Lexer, read_file, write_file, FILE_EXTENSION};
use open::that;
use std::{env, process::Command};

fn main() {
    let args: Vec<_> = env::args().collect();
    let raw_path = args.get(1);

    // compiler flags
    let print_tokens = args.contains(&"-t".to_string());
    let print_ast = args.contains(&"-a".to_string());
    let print_c_code = args.contains(&"-c".to_string());
    let clang = args.contains(&"-clang".to_string());
    let compile = !args.contains(&"-noc".to_string());

    // gcc or clang
    let compiler = if clang { "clang" } else { "gcc" };
    let out_path = "out.c";

    // reporting system
    match raw_path {
        Some(path) => {
            if path == "report" {
                handle_open("https://github.com/FloatingComet62/comrade/issues/new?assignees=&labels=&template=bug_report.md&title=");
                exit("", Some(0));
            }
            if path == "help" {
                println!(
                    "
comrade test{} -t
Print the tokens

comrade test{} -a
Print the AST generated

comrade test{} -c
Print the c code generated

comrade test{} -noc
Don't compile

comrade test{} -clang
Use clang to compile instead of gcc

comrade report
To report a bug

comrade help
To print this message
                ",
                    FILE_EXTENSION, FILE_EXTENSION, FILE_EXTENSION, FILE_EXTENSION, FILE_EXTENSION
                );
                exit("", Some(0));
            }
            let data = read_file(path);
            let parser = Lexer::new(data);
            let (_program, c_code) = parser.parse(true, print_tokens, print_ast, print_c_code);
            if !compile {
                return;
            }
            if let Err(e) = write_file(out_path, c_code) {
                exit(
                    &format!("Failed to write to {}\nError Trace:\n{}", out_path, e),
                    None,
                )
            }
            Command::new(compiler)
                .arg(out_path)
                .arg("-o main.exe")
                .spawn()
                .expect("Failed to compile C code");
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
