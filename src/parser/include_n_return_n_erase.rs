use super::{load, Node, NodeData, Statement};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    text: &String,
    identifiers: &mut Vec<Vec<String>>,
    enum_values: &mut Vec<Vec<String>>,
    struct_data: &mut Vec<Vec<String>>,
) -> usize {
    let mut params = data.1.clone();
    params.retain(|x| x != "->");
    program.push(Node::new(
        NodeData::Statement(Statement {
            action: text.to_string(),
            parameters: load(&params, identifiers, enum_values, struct_data),
        }),
        0,
        0,
    ));
    data.0 // skip to next and ignore the data
}
