use crate::FunctionCall;

use super::compiler;

pub fn compile(input: &FunctionCall, semi_colon_needed: bool) -> String {
    let mut output = String::new();
    for i in 0..input.identifier.len() {
        let item = &input.identifier[i];
        if item == "->" {
            output += "_";
        } else {
            output += item;
        }
        if i != input.identifier.len() - 1 {
            output += "_";
        }
    }
    output += "(";
    for item in &input.arguments {
        output += &compiler(&mut item.clone(), String::new(), false, true);
    }
    output += ")";
    if semi_colon_needed {
        output += ";";
    }
    output
}
