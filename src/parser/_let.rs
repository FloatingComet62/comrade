use crate::node;

use super::{get_till_token_or_block, load, Node, VariableAssignment};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    input: &Vec<String>,
    i: usize,
    previous_text: &String,
    mut identifiers: &mut Vec<Vec<String>>,
    mut first_identifiers: &mut Vec<String>,
) -> usize {
    let iden = get_till_token_or_block("=", &input, i, false);
    let raw_val = get_till_token_or_block("EOL", &input, iden.0, false);
    let val = load(&raw_val.1, &mut identifiers, &mut first_identifiers);
    identifiers.push(iden.1.clone());
    first_identifiers.push(iden.1[0].clone());
    // TODO: handle block ig
    program.push(node!(
        variable_assignment,
        VariableAssignment {
            identifier: iden.1,
            value: Box::new(val),
            immutability: false,
            publicity: previous_text == "public",
        }
    ));
    data.0 // skip to next and ignore the data
}
