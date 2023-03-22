use super::{get_till_token_or_block, load, Node, VariableAssignment};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    input: &Vec<String>,
    i: usize,
    previous_text: &String,
) -> usize {
    let iden = get_till_token_or_block("=", &input, i);
    let raw_val = get_till_token_or_block("EOL", &input, iden.0);
    let val = load(&raw_val.1);
    // TODO: handle block ig
    program.push(Node::new(
        None,
        None,
        None,
        Some(VariableAssignment {
            identifier: iden.1,
            value: Box::new(val),
            immutability: false,
            publicity: previous_text == "public",
        }),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ));
    data.0 // skip to next and ignore the data
}
