use crate::VariableAssignment;

use super::type_to_c_type;

pub fn compile(input: &VariableAssignment) -> String {
    let mut output = String::new();
    let blank = String::new();
    let type_data = &types(input).unwrap_or((blank.clone(), false));
    output += &type_data.0;
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
    if input.value.len() == 1 {
        if let Some(l) = &input.value[0].literal {
            return Ok(l.literal.clone());
        }
    } else {
        let mut output = "{ ".to_string();
        for (i, item) in input.value.iter().enumerate() {
            if let Some(l) = &item.literal {
                output += &l.literal;
                if i != input.value.len() - 1 {
                    output += ", ";
                }
            }
        }
        output += " }";
        return Ok(output);
    }
    Err(())
}

fn types(input: &VariableAssignment) -> Result<(String, bool), ()> {
    if input.value.len() == 1 {
        if let Some(l) = &input.value[0].literal {
            let res = type_to_c_type(&l.l_type);
            return Ok((res.0.to_string(), res.1));
        }
    } else {
        let mut output = "struct ".to_string();
        output += &input.type_data;
        return Ok((output, false));
    }
    Err(())
}
