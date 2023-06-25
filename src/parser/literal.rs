use super::{Literal, Node, NodeData, Types};

pub fn parser(program: &mut Vec<Node>, text: String, l_type: Types) {
    program.push(Node::new(
        NodeData::Literal(Literal {
            literal: text,
            l_type,
        }),
        0,
        0,
    ));
}
