use super::{get_till_token_or_block, Node, Struct, StructMember};
use crate::{node, type_from_str};

pub fn get_struct_member(i: usize, input: &Vec<String>) -> (usize, Option<StructMember>) {
    let mut case_data = get_till_token_or_block("->", &input, i, false);
    let mut type_data = get_till_token_or_block("EOL", &input, case_data.0, false);

    case_data.1.retain(|x| x != "EOL");
    type_data.1.retain(|x| x != "EOL");

    if type_data.1.len() == 0 {
        return (type_data.0, None);
    }

    return (
        type_data.0,
        Some(StructMember {
            identifier: case_data.1,
            t_mem: type_from_str(&type_data.1[0]),
        }),
    );
}

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    _identifiers: &mut Vec<Vec<String>>,
    _enum_values: &mut Vec<Vec<String>>,
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
