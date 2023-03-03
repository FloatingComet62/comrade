use super::{get_till_eol_or_block, load, Argument, Function, Node, Types};
use crate::exit;
use crate::type_from_str;

pub fn parser(program: &mut Vec<Node>, input: &Vec<String>, i: usize) -> usize {
    let data = get_till_eol_or_block(input, i);
    let mut getting_args = false;
    let mut args = vec![];
    let mut identifier = vec![];
    for cell in &data.1 {
        if cell == "(" {
            getting_args = true;
        } else if cell == ")" {
            break;
        }
        match getting_args {
            true => {
                if cell == "," || cell == ")" || cell == "(" {
                    continue;
                }
                let a_type = type_from_str(cell);
                match a_type {
                    Types::None => args.push(Argument {
                        identifier: cell.to_string(),
                        a_type: Types::None,
                    }),
                    x => {
                        let len = args.len();
                        args.get_mut(len - 1)
                            .unwrap_or_else(|| {
                                exit("Missing function argument identifier before type", None)
                            })
                            .a_type = x;
                    }
                }
            }
            false => identifier.push(cell.to_string()),
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
    ));
    data.0
}
