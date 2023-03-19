use super::{Node, Statement};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    text: &String,
) -> usize {
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
        None,
    ));
    data.0 // skip to next and ignore the data
}
