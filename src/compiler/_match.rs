use crate::{Expression, Match, Node, NodeData};

use super::compiler;

pub fn compile(input: &Match, semi_colon_needed: bool) -> String {
    let mut output = String::new();
    let to_eval = &compiler(&input.condition, String::new(), semi_colon_needed, false);
    for (i, case) in input.block.iter().enumerate() {
        if i == 0 {
            output += "if(";
            output += to_eval;
            output += " == ";
            output += &compiler(&case.case.clone(), String::new(), semi_colon_needed, false);
            output += ") {";
            output += &compiler(&case.block.clone(), String::new(), semi_colon_needed, false);
            output += "}";
            continue;
        }
        if case.case
            == vec![Node::new(
                NodeData::Expression(Expression {
                    expr: vec!["default".to_string()],
                }),
                0,
                0,
            )]
        {
            output += "else {";
            output += &compiler(&case.block.clone(), String::new(), semi_colon_needed, false);
            output += "}";
            continue;
        }
        output += "else if(";
        output += to_eval;
        output += " == ";
        output += &compiler(&case.case.clone(), String::new(), semi_colon_needed, false);
        output += ") {";
        output += &compiler(&case.block.clone(), String::new(), semi_colon_needed, false);
        output += "}";
    }
    output
}
