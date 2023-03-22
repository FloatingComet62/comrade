use super::{load, Argument, Function, Node, Types};
use crate::exit;
use crate::type_from_str;

pub fn parser(program: &mut Vec<Node>, data: (usize, Vec<String>, Vec<String>, bool)) -> usize {
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
    let nodes = load(&data.2);
    program.push(Node::new(
        None,
        Some(Function {
            identifier,
            arguments: args,
            return_type,
            nodes,
        }),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ));
    data.0
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
