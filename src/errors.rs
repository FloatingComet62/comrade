use colored::Colorize;

use crate::exit;

pub enum Errors {
    UNDEFINEDIDENTIFIER,
    UNDEFINEDFUNCTION,

    UNCLOSEDBLOCK,
    UNCLOSEDSTRING,

    MISSINGBLOCK,
    MISSINGMODULE,

    INCORRECTTYPE,
    INDEXOFFRANGE,
    IMMUTABLEVARIABLE,
}
fn type_to_msg(error: Errors) -> String {
    match error {
        Errors::UNDEFINEDIDENTIFIER => "Undefined Identifier",
        Errors::UNDEFINEDFUNCTION => "Undefined Function",

        Errors::UNCLOSEDBLOCK => "Unclosed Block",
        Errors::UNCLOSEDSTRING => "Unclosed String",

        Errors::MISSINGBLOCK => "Missing Block",
        Errors::MISSINGMODULE => "Missing Module",

        Errors::INCORRECTTYPE => "Incorrect Type",
        Errors::INDEXOFFRANGE => "Index out of range",
        Errors::IMMUTABLEVARIABLE => "Can't manipulate an immutable variable",
    }
    .to_string()
}

pub fn send_error(error_type: Errors, error_message: String, line: i32, column: i32) -> ! {
    println!(
        "{}\n{}\n\nLine: {}\nColumn: {}\n{}",
        type_to_msg(error_type).red().bold(),
        error_message.bold(),
        line,
        column,
        String::new()
    );
    exit("", Some(0));
}
