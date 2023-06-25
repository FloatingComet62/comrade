use crate::{exit, type_from_str, NodeData, Types};

use super::{get_till_token_or_block_and_math_block, load, Node, ParserData, VariableAssignment};

fn val_getter(
    i_type: &String,
    raw_val: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    (identifiers, enum_values, struct_data): ParserData,
    iden: &Vec<String>,
) -> Vec<Node> {
    let i_type_type = type_from_str(&i_type);
    if !(i_type_type == Types::None) {
        return load(&raw_val.1, identifiers, enum_values, struct_data);
    }
    // it's not list
    if i_type.is_empty() {
        return load(&raw_val.1, identifiers, enum_values, struct_data);
    }
    let val = load(&raw_val.2, identifiers, enum_values, struct_data);
    let mut self_data = struct_data.clone();
    self_data.retain(|x| x[0] == i_type.to_string()); // only 1 answer
    for (i, cell) in val.iter().enumerate() {
        match &cell.data {
            NodeData::Literal(_) => {
                let mut item_iden = iden.clone();
                let raw_member = self_data[0].get(i + 1);
                // at self_data[0][0] is the struct name
                if let Some(member) = raw_member {
                    item_iden.push(member.clone());
                    identifiers.push(item_iden);
                } else {
                    exit(
                        &format!("Unknown values of struct {} were passed", self_data[0][0]),
                        None,
                    )
                }
            }
            _ => todo!(),
        }
    }
    return val;
}

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    input: &Vec<String>,
    i: usize,
    previous_text: &String,
    (identifiers, enum_values, struct_data): ParserData,
    immutable: bool,
) -> usize {
    let raw_iden = get_till_token_or_block_and_math_block("=", input, i);
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
            continue;
        }
        i_type = item;
    }
    let raw_val = get_till_token_or_block_and_math_block("EOL", input, raw_iden.0);
    let val = val_getter(
        &i_type,
        raw_val,
        (identifiers, enum_values, struct_data),
        &iden,
    );

    identifiers.push(iden.clone());
    program.push(Node::new(
        NodeData::VariableAssignment(VariableAssignment {
            identifier: iden,
            value: Box::new(val),
            immutability: immutable,
            publicity: previous_text == "public",
            type_data: type_from_str(&i_type),
        }),
        0,
        0,
    ));
    data.0 // skip to next and ignore the data
}
