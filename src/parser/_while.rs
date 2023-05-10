use super::{load, ConditionBlock, Node, NodeData};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    identifiers: &mut Vec<Vec<String>>,
    enum_values: &mut Vec<Vec<String>>,
    struct_data: &mut Vec<Vec<String>>,
) -> usize {
    program.push(Node::new(
        NodeData::ConditionBlock(ConditionBlock {
            keyword: "while".to_string(),
            parameters: load(&data.1, identifiers, enum_values, struct_data),
            nodes: load(&data.2, identifiers, enum_values, struct_data),
        }),
        0,
        0,
    ));
    data.0 // skip to next and ignore the data
}
