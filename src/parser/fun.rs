use super::{load, Argument, Function, Node, ParserData, Types};
use crate::{
    errors::{send_error, Errors},
    exit, type_from_str, NodeData,
};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    (identifiers, enum_values, struct_data): ParserData,
) -> usize {
    // todo: maybe reimplement this with get_till_token_or_block ?
    let mut getting_args = false;
    let mut args = vec![];
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
    let mut fun_identifiers = identifiers.to_owned();
    fun_identifiers.append(&mut arg_identifiers);

    let nodes = load(&data.2, &mut fun_identifiers, enum_values, struct_data);

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
    data.0
}

fn arg_to_identifier(args: &Vec<Argument>) -> Vec<Vec<String>> {
    let mut output = vec![];

    for arg in args {
        output.push(vec![arg.identifier.clone()]);
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
