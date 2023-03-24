use crate::{Types, VariableAssignment};

pub fn parser(input: &VariableAssignment) -> String {
    let mut output = String::new();
    let blank = String::new();
    output += &types(input).unwrap_or(blank.clone());
    output += &input.identifier.join("_");
    output += " = ";
    output += &value(input).unwrap_or(blank.clone());
    output
}

fn value(input: &VariableAssignment) -> Result<String, ()> {
    if let Some(l) = &input.value[0].literal {
        return Ok(l.literal.clone());
    }
    Err(())
}

fn types(input: &VariableAssignment) -> Result<String, ()> {
    if let Some(l) = &input.value[0].literal {
        return Ok(match l.l_type {
            Types::Bool => "bool",
            Types::I32 => "int",
            Types::Str => "char*",
            _ => "",
        }
        .to_owned()
            + " ");
    }
    Err(())
}
