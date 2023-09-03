use comrade::{
    compiler,
    exit,
    lexer::Lexer,
    parser::{Parser, ParserData},
    read_file,
    // type_checker::{self},
    write_file,
    FILE_EXTENSION,
};
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
            let lexer = Lexer::new(data);
            let tokens = lexer.token_splitter();
            if print_tokens {
                println!("{:#?}", tokens);
            }

            // adding libs here so that they get recognized as identifiers
            // maybe if I make a no std version, I can just make identifiers just do this
            // ```rust
            // let mut identifiers: Vec<Vec<String>> = vec![];
            // ```

            let mut parser = Parser::new(tokens, ParserData::new(true));
            parser.load();

            if print_ast {
                println!("{:#?}", parser.program);
            }

            if !compile {
                exit("", Some(0));
            }

            // type_checker::check_main(&parser.program);

            let c_code = compiler::compiler(
                &parser.program,
                "
#include <stdbool.h>
        "
                .to_string(),
                true,
                false,
            );
            if print_c_code {
                println!("{:#?}", c_code);
            }
            if let Err(e) = write_file(out_path, c_code) {
                exit(
                    &format!("Failed to write to {}\nError Trace:\n{}", out_path, e),
                    None,
                )
            }
            Command::new(compiler)
                .arg(out_path)
                .arg("-omain.exe") // for some reason, using "-o main.exe" creates a file " main.exe"
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
