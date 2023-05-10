use super::{Literal, Node, NodeData, Types};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    text: &String,
) -> usize {
    program.push(Node::new(
        NodeData::Literal(Literal {
            literal: text.to_string(),
            l_type: Types::Bool,
        }),
        0,
        0,
    ));
    data.0 // skip to next and ignore the data
}
