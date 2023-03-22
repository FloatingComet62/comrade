use super::{Node, Statement};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    text: &String,
) -> usize {
    let mut params = data.1.clone();
    params.retain(|x| x != "->");
    program.push(Node::new(
        Some(Statement {
            action: text.to_string(),
            parameters: params,
        }),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ));
    data.0 // skip to next and ignore the data
}
