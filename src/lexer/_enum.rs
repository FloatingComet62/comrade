use super::{Enum, Node};

pub fn parser(program: &mut Vec<Node>, data: (usize, Vec<String>, Vec<String>, bool)) -> usize {
    let mut members = data.2.clone();
    members.retain(|x| !vec!["EOL", ",", "{", "}"].contains(&x.as_str()));
    program.push(Node::new(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Enum {
            identifier: data.1,
            members,
        }),
    ));
    data.0
}
