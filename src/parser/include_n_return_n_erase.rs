use crate::node;

use super::{load, Node, Statement};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    text: &String,
    mut identifiers: &mut Vec<Vec<String>>,
    mut enum_values: &mut Vec<Vec<String>>,
    mut struct_data: &mut Vec<Vec<String>>,
) -> usize {
    let mut params = data.1.clone();
    params.retain(|x| x != "->");
    program.push(node!(
        statement,
        Statement {
            action: text.to_string(),
            parameters: load(
                &params,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data
            ),
        }
    ));
    data.0 // skip to next and ignore the data
}
