use super::{Enum, Node, NodeData};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    _identifiers: &mut [Vec<String>],
    enum_values: &mut Vec<Vec<String>>,
    _struct_data: &mut [Vec<String>],
) -> usize {
    let mut members = data.2.clone();
    members.retain(|x| !vec!["EOL", ",", "{", "}"].contains(&x.as_str()));

    for member in members.iter() {
        enum_values.push(vec![data.1.join("_"), member.to_string()]);
    }
    program.push(Node::new(
        NodeData::Enum(Enum {
            identifier: data.1,
            members,
        }),
        0,
        0,
    ));
    data.0
}
