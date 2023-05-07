use super::{get_till_token_or_block_and_math_block, Node, Struct, StructMember};
use crate::{
    errors::{send_error, Errors},
    node, type_from_str, Types,
};

pub fn get_struct_member(i: usize, input: &Vec<String>) -> (usize, Option<StructMember>) {
    let mut case_data = get_till_token_or_block_and_math_block("->", input, i, false);
    let mut type_data = get_till_token_or_block_and_math_block("EOL", input, case_data.0, false);

    case_data.1.retain(|x| x != "EOL");
    type_data.1.retain(|x| x != "EOL");

    if type_data.1.is_empty() {
        return (type_data.0, None);
    }

    let t_mem = type_from_str(&type_data.1[0]);

    if t_mem == Types::None {
        send_error(
            Errors::UNDEFINEDIDENTIFIER,
            format!("Expected Type, found {}", &type_data.1[0]),
            0,
            0,
        );
    }

    (
        type_data.0,
        Some(StructMember {
            identifier: case_data.1,
            t_mem,
        }),
    )
}

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    _identifiers: &mut [Vec<String>],
    _enum_values: &mut [Vec<String>],
    struct_data: &mut Vec<Vec<String>>,
) -> usize {
    let mut block = vec![];
    let mut self_data = vec![data.1.join("_")];
    let mut j = 0;
    while j < data.2.len() {
        let x = get_struct_member(j, &data.2);
        if let Some(y) = x.1 {
            self_data.push(y.identifier.join("_"));
            block.push(y);
        }
        j = x.0;
    }

    struct_data.push(self_data);

    program.push(node!(
        _struct,
        Struct {
            identifier: data.1,
            members: block,
        }
    ));
    data.0 // skip to next and ignore the data
}
