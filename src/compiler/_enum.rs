use crate::Enum;

pub fn compile(input: &Enum) -> String {
    let mut output = String::new();
    let name = input.identifier.join("_");
    output += "enum ";
    output += &name;
    output += " {";
    for item in input.members.iter() {
        output += &name; // enum ENUM { ITEM } => enum ENUM { ENUM_ITEM } for uniqueness among different enums
        output += "_";
        output += item;
        output += ", ";
    }
    output += "};";
    output
}
