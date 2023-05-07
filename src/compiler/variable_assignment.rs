use crate::{type_from_str, Types, VariableAssignment};

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
        return compiler(&mut input.value, String::new(), false, false);
    }
    let mut output = "{ ".to_string();
    for (i, item) in input.value.iter().enumerate() {
        output += &compiler(&mut vec![item.clone()], String::new(), false, true);
        if i != input.value.len() - 1 {
            output += ", ";
        }
    }
    output += " }";
    output
}

fn types(input: &VariableAssignment) -> (String, bool) {
    if input.value.len() == 1 {
        if let Some(l) = &input.value[0].literal {
            let res = type_to_c_type(&l.l_type);
            return (res.0.to_string(), res.1);
        }
        let t = type_from_str(&input.type_data);
        let res = type_to_c_type(&t);
        return (res.0.to_string(), res.1);
    }
    let type_check = type_from_str(&input.type_data);
    if type_check == Types::None {
        let mut output = "struct ".to_string();
        output += &input.type_data;
        return (output, false);
    }
    let res = type_to_c_type(&type_check);
    (res.0.to_string(), res.1)
}
