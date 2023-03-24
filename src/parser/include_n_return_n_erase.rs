use crate::node;

use super::{load, Node, Statement};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    text: &String,
    mut identifiers: &mut Vec<Vec<String>>,
    mut first_identifiers: &mut Vec<String>,
) -> usize {
    let mut params = data.1.clone();
    params.retain(|x| x != "->");
    program.push(node!(
        statement,
        Statement {
            action: text.to_string(),
            parameters: load(&params, &mut identifiers, &mut first_identifiers),
        }
    ));
    data.0 // skip to next and ignore the data
}