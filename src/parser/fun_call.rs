use crate::node;

use super::{get_till_token_or_block, load, FunctionCall, Node};

pub fn parser(
    program: &mut Vec<Node>,
    text: &String,
    input: &Vec<String>,
    i: usize,
    mut identifiers: &mut Vec<Vec<String>>,
    mut first_identifiers: &mut Vec<String>,
) -> usize {
    let mut identifier = vec![text.to_string()];
    let mut raw_identifier = get_till_token_or_block("(", &input, i, false);

    identifier.append(&mut raw_identifier.1);

    let mut raw_args = vec![];
    let mut raw_raw_args = get_till_token_or_block(")", &input, raw_identifier.0 - 1, false);

    // basically, join the block you found with the main content
    raw_args.append(&mut raw_raw_args.1);
    raw_args.append(&mut raw_raw_args.2);

    let mut args: Vec<Vec<Node>> = vec![];
    let mut arg: Vec<String> = vec![];
    for item in &raw_args {
        if item == "," {
            if arg.len() > 0 {
                args.push(load(&arg, &mut identifiers, &mut first_identifiers));
            }
            arg = vec![];
            continue;
        }
        arg.push(item.to_string());
    }
    if arg.len() > 0 {
        args.push(load(&arg, &mut identifiers, &mut first_identifiers));
    }
    program.push(node!(
        function_call,
        FunctionCall {
            identifier,
            arguments: args,
        }
    ));
    raw_raw_args.0
}
