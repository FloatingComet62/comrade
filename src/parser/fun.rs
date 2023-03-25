use super::{load, Argument, Function, Node, Types};
use crate::exit;
use crate::lexer::get_first;
use crate::node;
use crate::type_from_str;

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    mut identifiers: &mut Vec<Vec<String>>,
    mut first_identifiers: &mut Vec<String>,
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
    let arg_identifiers = arg_to_identifier(&args);
    identifiers.append(&mut arg_identifiers.clone());
    let first_arg_identifiers = get_first(&arg_identifiers);
    first_identifiers.append(&mut first_arg_identifiers.clone());

    let nodes = load(&data.2, &mut identifiers, &mut first_identifiers);

    // remove identifiers
    identifiers.retain(|iden| {
        for arg_iden in arg_identifiers.iter() {
            if iden == arg_iden {
                return false;
            }
        }
        true
    });
    first_identifiers.retain(|iden| {
        for arg_iden in first_arg_identifiers.iter() {
            if iden == arg_iden {
                return false;
            }
        }
        true
    });

    program.push(node!(
        function,
        Function {
            identifier,
            arguments: args,
            return_type,
            nodes,
        }
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
    let mut output = args.clone();
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
