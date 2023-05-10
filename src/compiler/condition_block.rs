use crate::ConditionBlock;

use super::compiler;

pub fn compile(input: &ConditionBlock) -> String {
    let mut output = String::new();
    if input.keyword == "if" {
        output += "if(";
        output += &compiler(&input.parameters, String::new(), false, true);
        output += "){";
        output += &compiler(&input.nodes, String::new(), true, false);
        output += "}";
    }
    if input.keyword == "else if" {
        output += "else if(";
        output += &compiler(&input.parameters, String::new(), false, true);
        output += "){";
        output += &compiler(&input.nodes, String::new(), true, false);
        output += "}";
    }
    if input.keyword == "else" {
        output += "else {";
        output += &compiler(&input.nodes, String::new(), true, false);
        output += "}";
    }
    if input.keyword == "while" {
        output += "while (";
        output += &compiler(&input.parameters, String::new(), false, true);
        output += ") {";
        output += &compiler(&input.nodes, String::new(), true, false);
        output += "}";
    }
    output
}
