use super::{load, ConditionBlock, Node};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    mut identifiers: &mut Vec<Vec<String>>,
    mut first_identifiers: &mut Vec<String>,
) -> usize {
    program.push(Node::new(
        None,
        None,
        None,
        None,
        None,
        Some(ConditionBlock {
            keyword: "while".to_string(),
            parameters: load(&data.1, &mut identifiers, &mut first_identifiers),
            nodes: load(&data.2, &mut identifiers, &mut first_identifiers),
        }),
        None,
        None,
        None,
        None,
        None,
    ));
    data.0 // skip to next and ignore the data
}
