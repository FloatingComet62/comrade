use super::{load, ConditionBlock, Node};

pub fn parser(program: &mut Vec<Node>, data: (usize, Vec<String>, Vec<String>, bool)) -> usize {
    program.push(Node::new(
        None,
        None,
        None,
        None,
        None,
        Some(ConditionBlock {
            keyword: "while".to_string(),
            parameters: load(&data.1),
            nodes: load(&data.2),
        }),
        None,
        None,
        None,
        None,
    ));
    data.0 // skip to next and ignore the data
}
