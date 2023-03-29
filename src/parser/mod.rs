use crate::{
    node, str_list_to_string_list, type_from_str, Argument, ConditionBlock, Enum, Expression,
    ExternC, Function, FunctionCall, Literal, Match, MatchCase, Math, Node, NodeData, Statement,
    Struct, StructMember, StructValue, Types, VariableAssignment,
};
use std::fmt::Debug;

mod _const;
mod _enum;
mod _for;
mod _if;
mod _let;
mod _match;
mod _struct;
mod _while;
mod booleans;
mod extern_c;
mod fun;
mod fun_call;
mod include_n_return_n_erase;
mod math;

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

pub struct Lexer {
    pub splitted_text: Vec<String>,
    pub program: Vec<Node>,
    pub keywords: Vec<Vec<String>>,
    pub libs: Vec<Vec<String>>,
}
impl Lexer {
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
        if !is_comment {
            let operator = is_math(text.to_string());
            if let Ok(_) = &operator {
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
    return (j, output, block, got_block, math_block);
}

fn is_math(token: String) -> Result<(), ()> {
    if token == "+".to_string() {
        return Ok(());
    } else if token == "-".to_string() {
        return Ok(());
    } else if token == "*".to_string() {
        return Ok(());
    } else if token == "/".to_string() {
        return Ok(());
    } else if token == "+=".to_string() {
        return Ok(());
    } else if token == "-=".to_string() {
        return Ok(());
    } else if token == "*=".to_string() {
        return Ok(());
    } else if token == "/=".to_string() {
        return Ok(());
    } else if token == "=".to_string() {
        return Ok(());
    } else if token == "==".to_string() {
        return Ok(());
    } else if token == "!=".to_string() {
        return Ok(());
    } else if token == ">".to_string() {
        return Ok(());
    } else if token == "<".to_string() {
        return Ok(());
    } else if token == ">=".to_string() {
        return Ok(());
    } else if token == "<=".to_string() {
        return Ok(());
    } else if token == "(".to_string() {
        return Ok(());
    } else if token == ")".to_string() {
        return Ok(());
    } else {
        return Err(());
    }
}

pub fn load(
    input: &Vec<String>,
    mut identifiers: &mut Vec<Vec<String>>,
    mut enum_values: &mut Vec<Vec<String>>,
    mut struct_data: &mut Vec<Vec<String>>,
) -> Vec<Node> {
    let mut program = vec![];
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
            Mode::OR,
        ) {
            // identifier check
            for j in 0..identifiers.len() {
                if text == identifiers[j][0].as_str() {
                    let mut identifer = true;
                    for k in 0..identifiers[j].len() {
                        if let Some(to_check) = input.get(i + k) {
                            if to_check != identifiers[j][k].as_str() {
                                identifer = false;
                            }
                        } else {
                            // we reached the end of the file
                            break;
                        }
                    }
                    if identifer {
                        let mut idenf = identifiers[j].clone();
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
                        i += identifiers[j].len();
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
            i = _let::parser(
                &mut program,
                data,
                input,
                i,
                &previous_text,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
            );
        } else if text == "const" {
            i = _const::parser(
                &mut program,
                data,
                input,
                i,
                &previous_text,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
            );
        } else if text == "include" || text == "return" || text == "erase" {
            i = include_n_return_n_erase::parser(
                &mut program,
                data,
                text,
                identifiers,
                &mut enum_values,
                &mut struct_data,
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
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
            );
        } else if text == "while" {
            i = _while::parser(
                &mut program,
                data,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
            );
        // } else if text == "for" {
        // i = _for::parser(&mut program, data, &mut identifiers);
        } else if text == "struct" {
            i = _struct::parser(
                &mut program,
                data,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
            );
        } else if text == "enum" {
            i = _enum::parser(
                &mut program,
                data,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
            );
        } else if text == "fun" {
            i = fun::parser(
                &mut program,
                data,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
            );
        } else if has(
            &data.1,
            vec![
                "+", "-", "*", "/", "==", "=", ">", "<", "<=", ">=", "!=", "+=", "-=", "*=", "/=",
            ],
            Mode::OR,
        ) && !has(&data.1, vec!["let", "const"], Mode::OR)
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
                unblocked_op.len() != 0
            }
        {
            i = math::parser(
                &mut program,
                text,
                data,
                input,
                i,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
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
            i = _match::parser(
                &mut program,
                data,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
            );
        } else if has(&data.1, vec!["(", ")"], Mode::AND) {
            // also, it's a function call when there is no fun
            i = fun_call::parser(
                &mut program,
                text,
                input,
                i,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
            );
        }
        i += 1;
    }

    program
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

enum Mode {
    OR,
    AND,
}
fn has(data: &Vec<String>, things: Vec<&str>, mode: Mode) -> bool {
    match mode {
        Mode::AND => {
            for thing in things {
                if !data.contains(&thing.to_string()) {
                    return false;
                }
            }
            true
        }
        Mode::OR => {
            for thing in things {
                if data.contains(&thing.to_string()) {
                    return true;
                }
            }
            false
        }
    }
}
