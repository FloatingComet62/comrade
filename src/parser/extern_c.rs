use super::Node;
use crate::{ExternC, NodeData};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
) -> usize {
    program.push(Node::new(
        NodeData::ExternC(ExternC {
            block: format!("\n{}\n", data.1[0].clone()),
        }),
        0,
        0,
    ));
    data.0 // skip to next and ignore the data
}
