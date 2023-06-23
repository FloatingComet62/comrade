use crate::{lexer::Lexer, read_file, Node, NodeData, Statement, FILE_EXTENSION};

use super::compiler;

fn param_to_path(param: &[Node]) -> String {
    let mut path = String::new();
    let param_vec = match &param[0].data {
        NodeData::Expression(e) => &e.expr,
        _ => todo!(),
    };
    for i in 0..param_vec.len() {
        path += &param_vec[i];
        if i != param_vec.len() - 1 {
            path += "/";
        }
    }
    path += FILE_EXTENSION;
    path
}

fn param_to_iden_vec(params: &[Node], identifier: &[String]) -> Vec<String> {
    let mut output = vec![];
    let blank_str = &String::new();
    let binding = Node::blank();
    let expr = match &params.last().unwrap_or(&binding).data {
        NodeData::Expression(e) => &e.expr,
        _ => todo!(),
    };
    output.append(&mut vec![
        expr.last().unwrap_or(blank_str).to_string(),
        "_".to_string(),
    ]);
    output.append(&mut identifier.to_owned());
    output
}

pub fn compile(_program: &[Node], input: &Statement) -> String {
    let mut output = String::new();
    if input.action == "include" {
        let lib = param_to_path(&input.parameters);
        let parse = Lexer::new(read_file(&lib));
        let mut lib_tokens_to_parse = vec![];
        let (lib_tokens, _) = parse.parse(false, false, false, false);
        for i in 0..lib_tokens.len() {
            let raw_lib_token = &lib_tokens.get(i);
            if raw_lib_token.is_none() {
                continue;
            }
            let lib_token = &mut raw_lib_token.unwrap().clone();
            match &mut lib_token.data {
                NodeData::Function(func) => {
                    if func.identifier == vec!["__init__"] {
                        // __init__ is embedded directly and the function is removed
                        output += &compiler(&func.nodes, String::new(), true, false);
                    } else {
                        // normal function, add the path->function_name
                        //todo maybe with variables
                        let new_identifier = param_to_iden_vec(&input.parameters, &func.identifier);
                        func.identifier = new_identifier.clone();

                        lib_tokens_to_parse.push(lib_token.clone());
                    }
                }
                _ => todo!(),
            }
        }
        output += &compiler(&lib_tokens_to_parse, String::new(), true, false);
    } else {
        output += &input.action;
        output += " ";
        output += &compiler(&input.parameters, String::new(), true, false);
        output += ";";
    }
    output
}
