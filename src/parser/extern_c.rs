use super::Node;
use crate::{node, ExternC};

pub fn parser(program: &mut Vec<Node>, data: (usize, Vec<String>, Vec<String>, bool)) -> usize {
    program.push(node!(
        extern_c,
        ExternC {
            block: data.1[0].clone()
        }
    ));
    data.0 // skip to next and ignore the data
}
