use crate::node;

use super::{Literal, Node, Types};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    text: &String,
) -> usize {
    program.push(node!(
        literal,
        Literal {
            literal: text.to_string(),
            l_type: Types::Bool,
        }
    ));
    data.0 // skip to next and ignore the data
}
