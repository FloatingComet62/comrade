use crate::VariableAssignment;

use super::type_to_c_type;

pub fn compile(input: &VariableAssignment) -> String {
    let mut output = String::new();
    let blank = String::new();
    let type_data = &types(input).unwrap_or((blank.as_str(), false));
    output += type_data.0;
    output += " ";
    output += &input.identifier.join("_");
    if type_data.1 {
        output += "[]"
    }
    output += " = ";
    output += &value(input).unwrap_or(blank.clone());
    output += ";";
    output
}

fn value(input: &VariableAssignment) -> Result<String, ()> {
    if let Some(l) = &input.value[0].literal {
        return Ok(l.literal.clone());
    }
    Err(())
}

fn types(input: &VariableAssignment) -> Result<(&str, bool), ()> {
    if let Some(l) = &input.value[0].literal {
        return Ok(type_to_c_type(&l.l_type));
    }
    Err(())
}
