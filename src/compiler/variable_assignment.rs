use crate::{type_from_str, NodeData, Types, VariableAssignment};

use super::{compiler, type_to_c_type};

pub fn compile(input: &VariableAssignment) -> String {
    let mut output = String::new();
    let type_data = &types(input);
    output += &type_data.0;
    output += " ";
    output += &input.identifier.join("_");
    if type_data.1 {
        output += "[]"
    }
    output += " = ";
    output += &value(&mut input.clone());
    output += ";";
    output
}

fn value(input: &mut VariableAssignment) -> String {
    if input.value.len() == 1 {
        return compiler(&input.value, String::new(), false, false);
    }
    let mut output = "{ ".to_string();
    for (i, item) in input.value.iter().enumerate() {
        output += &compiler(&vec![item.clone()], String::new(), false, true);
        if i != input.value.len() - 1 {
            output += ", ";
        }
    }
    output += " }";
    output
}

fn types(input: &VariableAssignment) -> (String, bool) {
    if input.value.len() == 1 {
        match &input.value[0].data {
            NodeData::Literal(l) => {
                let res = type_to_c_type(&l.l_type);
                return (res.0.to_string(), res.1);
            }
            _ => {
                let res = type_to_c_type(&input.type_data);
                return (res.0.to_string(), res.1);
            }
        };
    }
    if input.type_data == Types::None {
        let mut output = "struct ".to_string();
        output += &format!("{:?}", &input.type_data);
        return (output, false);
    }
    let res = type_to_c_type(&input.type_data);
    (res.0.to_string(), res.1)
}
