use crate::node;

use super::{super::Operations, get_till_token_or_block_and_math_block, load, Math, Node};

pub fn parser(
    program: &mut Vec<Node>,
    text: &String,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    input: &Vec<String>,
    i: usize,
    identifiers: &mut Vec<Vec<String>>,
    enum_values: &mut Vec<Vec<String>>,
    struct_data: &mut Vec<Vec<String>>,
) -> usize {
    let mut operator = "";
    let mut operation = Operations::NULL;

    let mut inside_a_block = 0;
    for cell in data.4.iter() {
        if cell == "(" {
            inside_a_block += 1;
        }

        if inside_a_block == 0 {
            // aka, not in a block
            let temp_operation = token_to_op(cell);
            if temp_operation != Operations::NULL {
                operation = temp_operation;
                operator = cell;
                break;
            }
        }

        if cell == ")" {
            inside_a_block -= 1;
        }
    }

    let mut lhs = vec![text.to_string()];
    let mut raw_lhs = get_till_token_or_block_and_math_block(operator, input, i, false);
    let rhs = get_till_token_or_block_and_math_block("EOL", input, raw_lhs.0, false);
    lhs.append(&mut raw_lhs.1);

    program.push(node!(
        math,
        Math {
            lhs: load(&lhs, identifiers, enum_values, struct_data),
            rhs: load(&rhs.1, identifiers, enum_values, struct_data),
            operation,
        }
    ));
    rhs.0 // skip to next and ignore the data
}

fn token_to_op(token: &str) -> Operations {
    match token {
        "+" => Operations::ADD,
        "-" => Operations::SUB,
        "*" => Operations::MUL,
        "/" => Operations::DIV,
        "+=" => Operations::ADDEQT,
        "-=" => Operations::SUBEQT,
        "*=" => Operations::MULEQT,
        "/=" => Operations::DIVEQT,
        ">" => Operations::GR,
        "<" => Operations::LT,
        ">=" => Operations::EQGR,
        "<=" => Operations::EQLT,
        "=" => Operations::EQ,
        "==" => Operations::EQT,
        "!=" => Operations::NEQ,
        _ => Operations::NULL,
    }
}
