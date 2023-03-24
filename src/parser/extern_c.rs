use super::Node;
use crate::{node, ExternC};

pub fn parser(program: &mut Vec<Node>, data: (usize, Vec<String>, Vec<String>, bool)) -> usize {
    program.push(node!(
        extern_c,
        ExternC {
            block: join(&data.2)
        }
    ));
    data.0 // skip to next and ignore the data
}

fn join(input: &Vec<String>) -> String {
    let mut output = String::new();

    for item in input {
        if item == "EOL" {
            continue;
        }
        output += item;
    }

    output
}
