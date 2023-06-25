use crate::{
    str_list_to_string_list, type_from_str, Argument, ConditionBlock, Enum, Expression, Function,
    FunctionCall, Literal, Match, MatchCase, Math, Node, NodeData, Statement, Struct, StructMember,
    Types, VariableAssignment,
};

mod _enum;
mod _for;
mod _if;
mod _match;
mod _struct;
mod _while;
mod booleans;
mod extern_c;
mod fun;
mod fun_call;
mod literal;
mod math;
mod statement;
mod variable_assignment;

type ParserData<'a> = (
    &'a mut Vec<Vec<String>>,
    &'a mut Vec<Vec<String>>,
    &'a mut Vec<Vec<String>>,
);

pub struct Parser {
    pub splitted_text: Vec<String>,
    pub program: Vec<Node>,
    pub keywords: Vec<Vec<String>>,
    pub libs: Vec<Vec<String>>,
}
impl Parser {
    pub fn new(splitted_text: Vec<String>) -> Self {
        Self {
            splitted_text,
            program: vec![],
            keywords: vec![vec!["default".to_string()], vec!["NULL".to_string()]],
            libs: vec![
                str_list_to_string_list(vec!["std", "io"]),
                str_list_to_string_list(vec!["std", "math"]),
            ],
        }
    }
}

/*
    Returns:
    new index
    content till token
    block found
    & math block found
*/
pub fn get_till_token_or_block_and_math_block(
    token: &str,
    input: &Vec<String>,
    i: usize,
) -> (usize, Vec<String>, Vec<String>, bool, Vec<String>) {
    let mut output: Vec<String> = vec![];
    let mut block: Vec<String> = vec![];
    let mut math_block = vec![];
    let mut got_block = false;
    let mut getting_block = 0;
    let mut getting_function_call = 0;
    let mut is_comment = false;
    let mut j = i + 1;

    while j < input.len() {
        let text = &input[j];
        // ! must be first
        if text == "//" {
            is_comment = true;
        }
        if text == "EOL" {
            is_comment = false;
        }
        if is_comment {
            j += 1;
            continue;
        }
        let operator = is_math(text.to_string());
        if operator.is_ok() {
            math_block.push(text.to_string());
        }

        if text == "{" {
            getting_block += 1;
        }
        if text == "(" && token != "(" {
            // we are not getting the (, then get the entire function
            getting_function_call += 1;
        }
        if text == ")" {
            getting_function_call -= 1;
        }
        if text == token && getting_block == 0 && getting_function_call == 0 {
            break;
        }
        if getting_block > 0 {
            block.push(text.to_string());
        } else {
            output.push(text.to_string());
        }
        // ! must be last
        if text == "}" {
            got_block = true;
            getting_block -= 1;
            if getting_block == 0 && getting_function_call == 0 {
                // got a random block?
                // IT PROBABLY WAS IMPORTANT OR SOMETHING, ISN'T IT
                // stop everything, just return
                break;
            }
        }

        j += 1;
    }
    (j, output, block, got_block, math_block)
}

fn is_math(token: String) -> Result<(), ()> {
    let math_tokens = str_list_to_string_list(
        [
            "+", "-", "*", "/", "+=", "-=", "*=", "/=", "=", "==", "!=", ">", "<", ">=", "<=", "(",
            ")",
        ]
        .to_vec(),
    );
    for math_token in math_tokens.iter() {
        if &token == math_token {
            return Ok(());
        }
    }
    Err(())
}

fn list_check(idenf: &mut Vec<String>, input: &Vec<String>, i: usize) -> Option<()> {
    if let Some(first) = input.get(i + 1) {
        if !(first == "[") {
            return None;
        }
    }
    if let Some(second) = input.get(i + 3) {
        if !(second == "]") {
            return None;
        }
    }
    if let Some(middle) = input.get(i + 2) {
        let chars: Vec<char> = middle.chars().collect();
        if !chars[0].is_numeric() {
            return None;
        }
        // it is a list indexing
        idenf.append(&mut str_list_to_string_list(vec!["[", middle, "]"]));
        return Some(());
    }
    return None;
}

fn identifier_check(
    program: &mut Vec<Node>,
    i: usize,
    iden: &Vec<String>,
    input: &Vec<String>,
) -> Option<usize> {
    let mut identifer = true;
    let mut i_adder: usize = 0;
    for (k, id) in iden.iter().enumerate() {
        if let Some(to_check) = input.get(i + k) {
            if to_check != id.as_str() {
                identifer = false;
            }
        } else {
            // we reached the end of the file
            break;
        }
    }
    if !identifer {
        return None;
    }
    let mut idenf = iden.clone();
    if list_check(&mut idenf, input, i).is_some() {
        i_adder += 3;
    }
    program.push(Node::new(
        NodeData::Expression(Expression { expr: idenf }),
        0,
        0,
    ));
    i_adder += iden.len();
    Some(i_adder)
}

fn enum_check(
    program: &mut Vec<Node>,
    text: String,
    enum_val: &Vec<String>,
    input: &Vec<String>,
    i: usize,
) {
    if !(text == enum_val[0]) {
        return;
    }
    for (j, val) in enum_val.iter().enumerate() {
        if !(&input[i + j] == val) {
            continue;
        }
        program.push(Node::new(
            NodeData::Literal(Literal {
                literal: enum_val.join("_"),
                l_type: Types::I32,
            }),
            0,
            0,
        ))
    }
}

/// Generate the AST
/// * `identifiers` - Identifiers to look out for
/// * `enum_values` - Enum Values to look out for
/// * `struct_data` - Structs to look out for
pub fn load(
    input: &Vec<String>,
    identifiers: &mut Vec<Vec<String>>,
    enum_values: &mut Vec<Vec<String>>,
    struct_data: &mut Vec<Vec<String>>,
) -> Vec<Node> {
    let mut program = vec![];
    // previous token
    let mut previous_text;
    let mut text = &String::new();
    let mut i = 0;
    while i < input.len() {
        let data = get_till_token_or_block_and_math_block("EOL", input, i);
        previous_text = text.clone();
        text = &input[i];

        if text == "//" {
            i += data.0;
            continue;
        }
        // skip EOLs
        if text == "EOL" || text == "(" || text == ")" {
            i += 1;
            continue;
        }

        // no check if math
        if !has(
            &data.1,
            vec![
                "+", "-", "*", "/", "==", "=", ">", "<", "<=", ">=", "!=", "+=", "-=", "*=", "/=",
            ],
            Mode::Or,
        ) {
            // identifier check
            for iden in identifiers.iter() {
                if !(text == iden[0].as_str()) {
                    continue;
                }
                match identifier_check(&mut program, i, iden, input) {
                    None => {}
                    Some(x) => {
                        i += x;
                        break;
                    }
                }
            }
            // enum values
            for enum_val in enum_values.iter() {
                enum_check(&mut program, text.to_string(), enum_val, input, i);
            }
        }

        let extern_c_check = text == "externC";
        let variable_assignment_check = text == "let" || text == "const";
        let is_const = text == "const";
        let statement_text = text == "include" || text == "return" || text == "erase";
        let boolean_check = text == "true" || text == "false";
        let if_check = text == "if" || text == "else";
        let while_check = text == "while";
        let struct_check = text == "struct";
        let enum_check = text == "enum";
        let function_check = text == "fun";
        let math_check = has(
            &data.1,
            vec![
                "+", "-", "*", "/", "==", "=", ">", "<", "<=", ">=", "!=", "+=", "-=", "*=", "/=",
            ],
            Mode::Or,
        ) && !has(&data.1, vec!["let", "const"], Mode::Or)
            && {
                //checking if the math found is inside a block
                // eg. fib(x-1)
                // it's a function call, not a math
                let mut unblocked_op = vec![];
                let mut blocked = 0;
                for cell in data.4.iter() {
                    if cell == "(" {
                        blocked += 1;
                    }
                    if blocked == 0 {
                        unblocked_op.push(cell);
                    }
                    if cell == ")" {
                        blocked -= 1;
                    }
                }
                !unblocked_op.is_empty()
            };
        let string_check = text.chars().next().unwrap_or('\0') == '\"';
        let number_check = {
            let mut output = false;
            if text.len() >= 2
                && text.chars().next().expect("impossible") == '-'
                && text.chars().nth(1).expect("impossible").is_numeric()
            {
                output = true;
            }
            if !output {
                // there is only 1 previous true assignment for output, so if output is true, then that means it is -ve
                output = text.chars().next().unwrap_or('\0').is_numeric();
            }

            output
        };
        let type_check = type_from_str(text) != Types::None;
        let match_check = text == "match";
        let function_call_check = has(&data.1, vec!["(", ")"], Mode::And);

        // ! externC -> let -> math -> literal
        if extern_c_check {
            i = extern_c::parser(&mut program, data);
        } else if variable_assignment_check {
            i = variable_assignment::parser(
                &mut program,
                data,
                input,
                i,
                &previous_text,
                (identifiers, enum_values, struct_data),
                is_const,
            );
        } else if statement_text {
            i = statement::parser(
                &mut program,
                data,
                text,
                (identifiers, enum_values, struct_data),
            );
        } else if boolean_check {
            i = booleans::parser(&mut program, data, text);
        } else if if_check {
            i = _if::parser(
                &mut program,
                data,
                text,
                &previous_text,
                input,
                i,
                (identifiers, enum_values, struct_data),
            );
        } else if while_check {
            i = _while::parser(&mut program, data, (identifiers, enum_values, struct_data));
        // } else if text == "for" {
        // i = _for::parser(&mut program, data, &mut identifiers);
        } else if struct_check {
            i = _struct::parser(&mut program, data, (identifiers, enum_values, struct_data));
        } else if enum_check {
            i = _enum::parser(&mut program, data, (identifiers, enum_values, struct_data));
        } else if function_check {
            i = fun::parser(&mut program, data, (identifiers, enum_values, struct_data));
        } else if math_check {
            i = math::parser(
                &mut program,
                text,
                data,
                input,
                i,
                (identifiers, enum_values, struct_data),
            );
        } else if string_check {
            literal::parser(&mut program, text.to_string(), Types::Str);
        } else if number_check {
            // default number type is i32
            literal::parser(&mut program, text.to_string(), Types::I32);
        } else if type_check {
            literal::parser(&mut program, text.to_string(), Types::Type);
        } else if match_check {
            i = _match::parser(&mut program, data, (identifiers, enum_values, struct_data));
        } else if function_call_check {
            // also, it's a function call when there is no fun
            i = fun_call::parser(
                &mut program,
                text,
                input,
                i,
                (identifiers, enum_values, struct_data),
            );
        }
        i += 1;
    }

    program
}

enum Mode {
    Or,
    And,
}
fn has(data: &[String], things: Vec<&str>, mode: Mode) -> bool {
    match mode {
        Mode::And => {
            for thing in things {
                if !data.contains(&thing.to_string()) {
                    return false;
                }
            }
            true
        }
        Mode::Or => {
            for thing in things {
                if data.contains(&thing.to_string()) {
                    return true;
                }
            }
            false
        }
    }
}
