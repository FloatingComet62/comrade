use super::{Literal, Node, Types};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    text: &String,
) -> usize {
    program.push(Node::new(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Literal {
            literal: text.to_string(),
            l_type: Types::Bool,
        }),
        None,
    ));
    data.0 // skip to next and ignore the data
}
