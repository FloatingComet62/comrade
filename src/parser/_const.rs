use crate::node;

use super::{get_till_token_or_block, load, Node, VariableAssignment};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    input: &Vec<String>,
    i: usize,
    previous_text: &String,
    mut identifiers: &mut Vec<Vec<String>>,
    mut enum_values: &mut Vec<Vec<String>>,
    mut struct_data: &mut Vec<Vec<String>>,
) -> usize {
    let raw_iden = get_till_token_or_block("=", &input, i, false);
    let mut iden = vec![];
    let mut i_type = String::new();
    let mut getting_iden = true;
    for item in raw_iden.1 {
        if item == "->" {
            getting_iden = false;
            continue;
        }
        if getting_iden {
            iden.push(item);
        } else {
            i_type = item;
        }
    }
    let raw_val = get_till_token_or_block("EOL", &input, raw_iden.0, false);
    let val = load(
        &raw_val.1,
        &mut identifiers,
        &mut enum_values,
        &mut struct_data,
    );
    identifiers.push(iden.clone());
    // TODO: handle block ig
    program.push(node!(
        variable_assignment,
        VariableAssignment {
            identifier: iden,
            value: Box::new(val),
            immutability: true,
            publicity: previous_text == "public",
            type_data: i_type,
        }
    ));
    data.0 // skip to next and ignore the data
}
