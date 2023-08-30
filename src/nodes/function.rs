use crate::{
    compiler::{compiler, type_to_c_type},
    errors::{send_error, Errors},
    exit,
    parser::{Parser, ParserData},
    type_from_str, Argument, Function, Node, NodeData, Types,
};

use super::NodeInterferace;

fn arg_to_identifier(args: &Vec<Argument>) -> Vec<Vec<String>> {
    let mut output = vec![];

    for arg in args {
        output.push(vec![arg.identifier.to_string()]);
    }

    output
}

fn handle_a_type(args: Vec<Argument>, cell: &String, a_type: Types) -> Vec<Argument> {
    let mut output = args;
    match a_type {
        Types::None => output.push(Argument {
            identifier: cell.to_string(),
            a_type: Types::None,
        }),
        x => {
            let len = output.len();
            output
                .get_mut(len - 1)
                .unwrap_or_else(|| exit("Missing function argument identifier before type", None))
                .a_type = x;
        }
    }
    output
}

pub struct FunctionManager {}
impl FunctionManager {
    pub fn new() -> Self {
        Self {}
    }
}
impl NodeInterferace<Function> for FunctionManager {
    fn check(&self, text: String) -> bool {
        text == "fun"
    }
    fn parser(
        &self,
        _parser: Parser,
        program: &mut Vec<crate::Node>,
        data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
        _text: &String,
        _previous_text: &String,
        _input: &Vec<String>,
        i: &mut usize,
        parser_data: &mut ParserData,
    ) {
        // todo: maybe reimplement this with get_till_token_or_block ?
        let mut getting_args = false;
        let mut args: Vec<Argument> = vec![];
        let mut identifier = vec![];
        for cell in &data.1 {
            if cell == "->" {
                continue;
            }
            if cell == "(" {
                getting_args = true;
            } else if cell == ")" {
                break;
            }
            if getting_args {
                if cell == "," || cell == ")" || cell == "(" {
                    continue;
                }
                let a_type = type_from_str(cell);
                args = handle_a_type(args, cell, a_type)
            } else {
                identifier.push(cell.to_string());
            }
        }
        let return_type = type_from_str(&data.1[data.1.len() - 1]);

        // update identifiers
        let mut arg_identifiers = arg_to_identifier(&args);
        let mut fun_identifiers = parser_data.identifier.to_owned();
        fun_identifiers.append(&mut arg_identifiers);

        let nodes = {
            let mut parser = Parser::new(
                data.2,
                ParserData {
                    identifier: fun_identifiers,
                    enum_values: parser_data.enum_values.clone(),
                    struct_data: parser_data.struct_data.clone(),
                },
            );
            parser.load();
            parser.program
        };

        // checking if an argument is missing type anotation
        for arg in args.iter() {
            if arg.a_type == Types::None {
                send_error(
                    Errors::UNDEFINEDIDENTIFIER,
                    format!(
                        "Argument {} of Function {} is missing type anotation",
                        arg.identifier,
                        identifier.join("->")
                    ),
                    0,
                    0,
                );
            }
        }

        // original identifiers is untouched and all the variables are inside fun_identifiers
        // which is gonna die after this function call ends

        program.push(Node::new(
            NodeData::Function(Function {
                identifier,
                arguments: args,
                return_type,
                nodes,
            }),
            0,
            0,
        ));
        *i = data.0
    }
    fn compiler(
        &self,
        data: Function,
        _semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        let mut output = String::new();
        let type_data = type_to_c_type(&data.return_type);
        output += type_data.0;
        if type_data.1 {
            output += "[]"
        }
        output += " ";
        output += &data.identifier.join("_");
        output += "(";
        for (i, item) in data.arguments.iter().enumerate() {
            let type_data = type_to_c_type(&item.a_type);
            output += type_data.0;
            output += " ";
            output += &item.identifier;
            if type_data.1 {
                output += "[]"
            }
            if i != data.arguments.len() - 1 {
                output += ", ";
            }
        }
        output += ") {\n";
        output += &compiler(&data.nodes, String::new(), true, false);
        output += "}";
        Some(output)
    }
}
