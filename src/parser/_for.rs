use super::{load, ConditionBlock, Node};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    mut identifiers: &mut Vec<Vec<String>>,
    mut first_identifiers: &mut Vec<String>,
) -> usize {
    let mut raw_params = vec![];
    for thingy in data.1.iter() {
        if thingy == "/" {
            raw_params.push("EOL".to_string());
            continue;
        }
        raw_params.push(thingy.to_string());
    }
    program.push(Node::new(
        None,
        None,
        None,
        None,
        None,
        Some(ConditionBlock {
            keyword: "function".to_string(),
            parameters: load(&raw_params, &mut identifiers, &mut first_identifiers),
            nodes: load(&data.2, &mut identifiers, &mut first_identifiers),
        }),
        None,
        None,
        None,
        None,
        None,
    ));
    data.0
}
