use crate::Function;

use super::{compiler, type_to_c_type};

pub fn compile(input: &mut Function) -> String {
    let mut output = String::new();
    let type_data = type_to_c_type(&input.return_type);
    output += type_data.0;
    if type_data.1 {
        output += "[]"
    }
    output += " ";
    output += &input.identifier.join("_");
    output += "(";
    for (i, item) in input.arguments.iter().enumerate() {
        let type_data = type_to_c_type(&item.a_type);
        output += type_data.0;
        output += " ";
        output += &item.identifier;
        if type_data.1 {
            output += "[]"
        }
        if i != input.arguments.len() - 1 {
            output += ", ";
        }
    }
    output += ") {\n";
    output += &compiler(&mut input.nodes, false);
    output += "}";
    output
}
