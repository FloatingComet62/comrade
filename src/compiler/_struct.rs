use crate::{compiler::type_to_c_type, Struct, StructMember};

pub fn compile(input: &Struct) -> String {
    let mut output = "struct ".to_string();
    output += &input.identifier.join("_");
    output += "{";
    for item in input.members.iter() {
        let type_data = &types(item);
        output += &type_data.0;
        output += " ";
        output += &item.identifier.join("_");
        if type_data.1 {
            output += "[]"
        }
        output += ";";
    }
    output += "}";
    output += ";";
    output
}

fn types(input: &StructMember) -> (String, bool) {
    let res = type_to_c_type(&input.t_mem);
    (res.0.to_string(), res.1)
}
