use super::{get_till_token_or_block, Node, Statement};

pub fn parser(program: &mut Vec<Node>, text: &String, input: &Vec<String>, i: usize) -> usize {
    let data = get_till_token_or_block("EOL", &input, i);
    program.push(Node::new(
        Some(Statement {
            action: text.to_string(),
            parameters: data.1,
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
