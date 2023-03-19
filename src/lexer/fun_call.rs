use super::{get_till_token_or_block, load, FunctionCall, Node};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    i: usize,
) -> usize {
    let raw_identifier = get_till_token_or_block("(", &data.1, i);
    let raw_args = get_till_token_or_block(")", &data.1, raw_identifier.0);
    let mut args: Vec<Vec<Node>> = vec![];
    let mut arg: Vec<String> = vec![];
    for item in &raw_args.1 {
        if item == "," {
            args.push(load(&arg));
            arg = vec![];
            continue;
        }
        arg.push(item.to_string());
    }
    program.push(Node::new(
        None,
        None,
        Some(FunctionCall {
            identifier: raw_identifier.1,
            arguments: args,
        }),
        None,
        None,
        None,
        None,
        None,
    ));
    raw_args.0
}
