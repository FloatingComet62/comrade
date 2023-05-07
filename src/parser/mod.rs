use crate::{
    errors::{send_error, Errors},
    node, str_list_to_string_list, type_from_str, Argument, ConditionBlock, Enum, Expression,
    ExternC, Function, FunctionCall, Literal, Match, MatchCase, Math, Node, NodeData, Statement,
    Struct, StructMember, StructValue, Types, VariableAssignment,
};
use std::fmt::Debug;

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
mod include_n_return_n_erase;
mod math;
mod variable_assignment;

impl Debug for Node {
    // don't display n_type
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.data))
    }
}
impl Node {
    pub fn new(
        statement: Option<Statement>,
        function: Option<Function>,
        function_call: Option<FunctionCall>,
        variable_assignment: Option<VariableAssignment>,
        expression: Option<Expression>,
        condition_block: Option<ConditionBlock>,
        _match: Option<Match>,
        literal: Option<Literal>,
        m: Option<Math>,
        s: Option<Struct>,
        e: Option<Enum>,
        e_c: Option<ExternC>,
        struct_value: Option<StructValue>,
    ) -> Self {
        Self {
            data: NodeData {
                statement,
                function,
                function_call,
                variable_assignment,
                expression,
                condition_block,
                _match,
                literal,
                math: m,
                _struct: s,
                _enum: e,
                extern_c: e_c,
                struct_value,
            },
        }
    }
    pub fn blank() -> Node {
        Node::new(
            None, None, None, None, None, None, None, None, None, None, None, None, None,
        )
    }
}

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
    back: bool,
) -> (usize, Vec<String>, Vec<String>, bool, Vec<String>) {
    let mut output: Vec<String> = vec![];
    let mut block: Vec<String> = vec![];
    let mut math_block = vec![];
    let mut got_block = false;
    let mut getting_block = 0;
    let mut getting_function_call = 0;
    let mut is_comment = false;
    let mut j = i;
    let mut oh_god_we_reached_the_start_while_going_back = false;
    if back {
        j = j.checked_sub(1).unwrap_or_else(|| {
            oh_god_we_reached_the_start_while_going_back = true;
            0
        });
        send_error(Errors::MISSINGBLOCK, String::new(), 0, 0);
    } else {
        j += 1;
    }

    if oh_god_we_reached_the_start_while_going_back {
        return (j, output, block, got_block, math_block);
    }

    while j < input.len() {
        let text = &input[j];
        // ! must be first
        if text == "#" {
            is_comment = true;
        }
        if text == "EOL" {
            is_comment = false;
        }
        if is_comment {
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

        if back {
            j = j.checked_sub(1).unwrap_or_else(|| {
                oh_god_we_reached_the_start_while_going_back = true;
                0
            });
        } else {
            j += 1;
        }
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
        let data = get_till_token_or_block_and_math_block("EOL", input, i, false);
        previous_text = text.clone();
        text = &input[i];

        if text == "#" {
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
                if text == iden[0].as_str() {
                    let mut identifer = true;
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
                    if identifer {
                        let mut idenf = iden.clone();
                        if let Some(first) = input.get(i + 1) {
                            if first == "[" {
                                if let Some(second) = input.get(i + 3) {
                                    if second == "]" {
                                        if let Some(middle) = input.get(i + 2) {
                                            let chars: Vec<char> = middle.chars().collect();
                                            if is_digit(chars[0]) {
                                                // it is a list indexing
                                                idenf.append(&mut str_list_to_string_list(vec![
                                                    "[", middle, "]",
                                                ]));
                                                i += 3;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        program.push(node!(expression, Expression { expr: idenf }));
                        i += iden.len();
                        break;
                    }
                }
            }
            // enum values
            for _enum in enum_values.iter() {
                if text == &_enum[0] {
                    for (j, val) in _enum.iter().enumerate() {
                        if &input[i + j] == val {
                            program.push(node!(
                                literal,
                                Literal {
                                    literal: _enum.join("_"),
                                    l_type: Types::I32
                                }
                            ))
                        }
                    }
                }
            }
        }

        // ! externC -> let -> math -> literal -> others
        if text == "externC" {
            i = extern_c::parser(&mut program, data);
        } else if text == "let" {
            i = variable_assignment::parser(
                &mut program,
                data,
                input,
                i,
                &previous_text,
                identifiers,
                enum_values,
                struct_data,
                false,
            );
        } else if text == "const" {
            i = variable_assignment::parser(
                &mut program,
                data,
                input,
                i,
                &previous_text,
                identifiers,
                enum_values,
                struct_data,
                true,
            );
        } else if text == "include" || text == "return" || text == "erase" {
            i = include_n_return_n_erase::parser(
                &mut program,
                data,
                text,
                identifiers,
                enum_values,
                struct_data,
            );
        } else if text == "true" || text == "false" {
            i = booleans::parser(&mut program, data, text);
        } else if text == "if" || text == "else" {
            i = _if::parser(
                &mut program,
                data,
                text,
                &previous_text,
                input,
                i,
                identifiers,
                enum_values,
                struct_data,
            );
        } else if text == "while" {
            i = _while::parser(&mut program, data, identifiers, enum_values, struct_data);
        // } else if text == "for" {
        // i = _for::parser(&mut program, data, &mut identifiers);
        } else if text == "struct" {
            i = _struct::parser(&mut program, data, identifiers, enum_values, struct_data);
        } else if text == "enum" {
            i = _enum::parser(&mut program, data, identifiers, enum_values, struct_data);
        } else if text == "fun" {
            i = fun::parser(&mut program, data, identifiers, enum_values, struct_data);
        } else if has(
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
            }
        {
            i = math::parser(
                &mut program,
                text,
                data,
                input,
                i,
                identifiers,
                enum_values,
                struct_data,
            );
        } else if text.chars().next().unwrap_or('\0') == '\"' {
            program.push(node!(
                literal,
                Literal {
                    literal: text.to_string(),
                    l_type: Types::Str,
                }
            ));
        } else if is_digit(text.chars().next().unwrap_or('\0')) {
            program.push(node!(
                literal,
                Literal {
                    literal: text.to_string(),
                    l_type: Types::I32, // * i32 is the default number
                }
            ));
        } else if type_from_str(text) != Types::None {
            program.push(node!(
                literal,
                Literal {
                    literal: text.to_string(),
                    l_type: Types::Type, // * i32 is the default number
                }
            ));
        } else if text == "match" {
            i = _match::parser(&mut program, data, identifiers, enum_values, struct_data);
        } else if has(&data.1, vec!["(", ")"], Mode::And) {
            // also, it's a function call when there is no fun
            i = fun_call::parser(
                &mut program,
                text,
                input,
                i,
                identifiers,
                enum_values,
                struct_data,
            );
        }
        i += 1;
    }

    program
}

fn is_digit(c: char) -> bool {
    ('0'..='9').contains(&c)
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
