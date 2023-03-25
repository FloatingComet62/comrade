use crate::ConditionBlock;

use super::compiler;

pub fn compile(input: &mut ConditionBlock) -> String {
    let mut output = String::new();
    if input.keyword == "if" {
        output += "if(";
        output += &compiler(&mut input.parameters, false);
        output += "){";
        output += &compiler(&mut input.nodes, false);
        output += "}";
    }
    if input.keyword == "else if" {
        output += "else if(";
        output += &compiler(&mut input.parameters, false);
        output += "){";
        output += &compiler(&mut input.nodes, false);
        output += "}";
    }
    if input.keyword == "else" {
        output += "else {";
        output += &compiler(&mut input.nodes, false);
        output += "}";
    }
    output
}
