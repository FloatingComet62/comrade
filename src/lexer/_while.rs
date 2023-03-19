use super::{get_till_token_or_block, load, ConditionBlock, Node};

pub fn parser(program: &mut Vec<Node>, input: &Vec<String>, i: usize) -> usize {
    let data = get_till_token_or_block("EOL", &input, i);
    program.push(Node::new(
        None,
        None,
        None,
        None,
        None,
        Some(ConditionBlock {
            keyword: "while".to_string(),
            parameters: data.1,
            nodes: load(&data.2),
        }),
        None,
        None,
    ));
    data.0 // skip to next and ignore the data
}
