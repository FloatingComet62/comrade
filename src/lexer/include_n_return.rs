use super::{get_till_eol_or_block, Node, Statement};

pub fn parser(program: &mut Vec<Node>, text: &String, input: &Vec<String>, i: usize) -> usize {
    let data = get_till_eol_or_block(&input, i);
    program.push(Node::new(
        Some(Statement {
            action: text.to_string(),
            parameters: data.1,
        }),
        None,
        None,
    ));
    data.0 // skip to next and ignore the data
}
