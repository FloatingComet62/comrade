use crate::node;

use super::{load, ConditionBlock, Node};

#[allow(dead_code)]
pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    identifiers: &mut Vec<Vec<String>>,
    enum_values: &mut Vec<Vec<String>>,
    struct_data: &mut Vec<Vec<String>>,
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
            parameters: load(&raw_params, identifiers, enum_values, struct_data),
            nodes: load(&data.2, identifiers, enum_values, struct_data),
        }
    ));
    data.0
}
