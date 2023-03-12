use super::{Argument, ArgumentLiteral, FunctionCall, Literal, Node, Types};
use crate::exit;

fn is_digit(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>),
    text: &String,
) -> usize {
    let mut getting_args = false;
    let mut args = vec![];
    let mut identifier = vec![text.to_string()];
    for cell in &data.1 {
        if cell == "(" {
            getting_args = true;
        } else if cell == ")" {
            break;
        }
        if getting_args {
            if cell == "," || cell == ")" || cell == "(" {
                continue;
            }
            let first_char = cell
                .chars()
                .next()
                .unwrap_or_else(|| exit("Blank function call argument", None));
            let d = is_digit(first_char);
            if d || first_char == '\"' {
                args.push(ArgumentLiteral {
                    argument: None,
                    literal: Some(Literal {
                        l_type: if d { Types::I32 } else { Types::Str },
                        literal: cell.to_string(),
                    }),
                })
            } else {
                args.push(ArgumentLiteral {
                    argument: Some(Argument {
                        identifier: cell.to_string(),
                        a_type: Types::None, // figure out argument
                    }),
                    literal: None,
                })
            }
        } else {
            identifier.push(cell.to_string());
        }
    }
    program.push(Node::new(
        None,
        None,
        Some(FunctionCall {
            identifier,
            arguments: args,
        }),
    ));
    data.0
}
