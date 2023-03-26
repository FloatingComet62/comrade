use crate::node;

use super::{load, ConditionBlock, Node};

#[allow(dead_code)]
pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    mut identifiers: &mut Vec<Vec<String>>,
    mut enum_values: &mut Vec<Vec<String>>,
    mut struct_data: &mut Vec<Vec<String>>,
) -> usize {
    let mut raw_params = vec![];
    for thingy in data.1.iter() {
        if thingy == "/" {
            raw_params.push("EOL".to_string());
            continue;
        }
        raw_params.push(thingy.to_string());
    }
    program.push(node!(
        condition_block,
        ConditionBlock {
            keyword: "function".to_string(),
            parameters: load(
                &raw_params,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data
            ),
            nodes: load(
                &data.2,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data
            ),
        }
    ));
    data.0
}
