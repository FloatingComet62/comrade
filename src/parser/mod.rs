use crate::Types;
use std::{fmt::Debug, ops::Deref};

mod _const;
mod _enum;
mod _for;
mod _if;
mod _let;
mod _match;
mod _struct;
mod _while;
mod booleans;
mod fun;
mod fun_call;
mod include_n_return_n_erase;
mod math;

#[derive(Debug)]
pub enum Operations {
    NULL,

    ADD,    // addition              "+"
    SUB,    // subtraction           "-"
    MUL,    // multiplication        "*"
    DIV,    // division              "/"
    EQ,     // equal                 "=="
    EQGR,   // equal or greater than ">="
    EQLT,   // equal or less than    "<="
    GR,     // greater than          ">"
    LT,     // less than             "<"
    NEQ,    // not equal             "!="
    EQT,    // equate to rhs         "="
    ADDEQT, // add rhs to lhs        "+="
    SUBEQT, // subtract rhs to lhs   "-="
    MULEQT, // multiply rhs to lhs   "*="
    DIVEQT, // divide rhs to lhs     "/="
}
#[derive(Debug, Clone)]
pub struct Argument {
    pub identifier: String,
    pub a_type: Types,
}
#[derive(Debug)]
pub struct Literal {
    pub literal: String,
    pub l_type: Types,
}
#[derive(Debug)]
pub struct Statement {
    pub action: String,
    pub parameters: Vec<String>, //todo Vec<Node>
}
#[derive(Debug)]
pub struct Function {
    pub identifier: Vec<String>,
    pub return_type: Types,
    pub arguments: Vec<Argument>,
    pub nodes: Vec<Node>,
}
#[derive(Debug)]
pub struct FunctionCall {
    pub identifier: Vec<String>,
    pub arguments: Vec<Vec<Node>>,
}
#[derive(Debug)]
pub struct VariableAssignment {
    pub identifier: Vec<String>,
    pub value: Box<Vec<Node>>,
    pub immutability: bool,
    pub publicity: bool,
}
#[derive(Debug)]
pub struct Expression {
    pub expr: Vec<String>, // maybe node? idk
}
#[derive(Debug)]
pub struct ConditionBlock {
    pub keyword: String,
    pub parameters: Vec<Node>,
    pub nodes: Vec<Node>,
}
#[derive(Debug)]
pub struct MatchCase {
    pub case: Vec<String>, // maybe something else, eg. ["addition", or, "add", or "a"]
    pub block: Vec<Node>,
}
#[derive(Debug)]
pub struct Match {
    pub condition: Vec<Node>,
    pub block: Vec<MatchCase>,
}
#[derive(Debug)]
pub struct StructMember {
    pub identifier: Vec<String>,
    pub t_mem: Types,
}
#[derive(Debug)]
pub struct Struct {
    pub identifier: Vec<String>,
    pub members: Vec<StructMember>,
}
#[derive(Debug)]
pub struct Enum {
    pub identifier: Vec<String>,
    pub members: Vec<String>,
}
#[derive(Debug)]
pub struct Math {
    pub lhs: Vec<Node>,
    pub rhs: Vec<Node>,
    pub operation: Operations,
}
//todo identifier ?
pub struct NodeData {
    pub statement: Option<Statement>,
    pub function: Option<Function>,
    pub function_call: Option<FunctionCall>,
    pub variable_assignment: Option<VariableAssignment>,
    pub expression: Option<Expression>,
    pub condition_block: Option<ConditionBlock>,
    pub _match: Option<Match>,
    pub literal: Option<Literal>,
    pub math: Option<Math>,
    pub _struct: Option<Struct>,
    pub _enum: Option<Enum>,
}
impl Debug for NodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! check {
            ($x: expr) => {
                if let Some(x) = $x {
                    return f.write_str(&format!("{:?}", x));
                }
            };
        }

        check!(&self.statement);
        check!(&self.function);
        check!(&self.function_call);
        check!(&self.variable_assignment);
        check!(&self.expression);
        check!(&self.condition_block);
        check!(&self._match);
        check!(&self.literal);
        check!(&self.math);
        check!(&self._struct);
        check!(&self._enum);

        f.write_str("{}")
    }
}
// todo: if by the end of the parser, all node has is "data", just make Node NodeData
pub struct Node {
    pub data: NodeData,
}
impl Deref for Node {
    type Target = NodeData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
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
            },
        }
    }
    pub fn blank() -> Node {
        Node::new(
            None, None, None, None, None, None, None, None, None, None, None,
        )
    }
}

pub struct Lexer {
    pub splitted_text: Vec<String>,
    pub program: Vec<Node>,
    pub keywords: Vec<String>,
}
impl Lexer {
    pub fn new(splitted_text: Vec<String>) -> Self {
        Self {
            splitted_text,
            program: vec![],
            keywords: vec![
                String::from("include"),
                String::from("fun"),
                String::from("return"),
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
pub fn get_till_token_or_block(
    token: &str,
    input: &Vec<String>,
    i: usize,
    back: bool,
) -> (usize, Vec<String>, Vec<String>, bool) {
    let mut output: Vec<String> = vec![];
    let mut block: Vec<String> = vec![];
    let mut got_block = false;
    let mut getting_block = 0;
    let mut getting_function_call = false;
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
        return (j, output, block, got_block);
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
            if text == "{" {
                getting_block += 1;
            }
            if text == "(" && token != "(" {
                // we are not getting the (, then get the entire function
                getting_function_call = true;
            }
            if text == token && getting_block == 0 && !getting_function_call {
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
                if getting_block == 0 {
                    // got a random block?
                    // IT PROBABLY WAS IMPORTANT OR SOMETHING, ISN'T IT
                    // stop everything, just return
                    break;
                }
            }
            if text == ")" {
                getting_function_call = false;
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
    return (j, output, block, got_block);
}

pub fn load(
    input: &Vec<String>,
    mut identifiers: &mut Vec<Vec<String>>,
    mut first_identifiers: &mut Vec<String>,
) -> Vec<Node> {
    let mut program = vec![];
    let mut previous_text;
    let mut text = &String::new();
    let mut i = 0;
    while i < input.len() {
        let data = get_till_token_or_block("EOL", input, i, false);
        previous_text = text.clone();
        text = &input[i];

        if text == "#" {
            i += data.0;
            continue;
        }
        // skip EOLs
        if text == "EOL" {
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
            for j in 0..first_identifiers.len() {
                if text == first_identifiers[j].as_str() {
                    let mut identifer = true;
                    for k in 0..identifiers[j].len() {
                        if &input[i + k] != identifiers[j][k].as_str() {
                            identifer = false;
                        }
                    }
                    if identifer {
                        program.push(Node::new(
                            None,
                            None,
                            None,
                            None,
                            Some(Expression {
                                expr: identifiers[j].clone(),
                            }),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        ));
                        i += identifiers[j].len();
                        continue;
                    }
                }
            }
        }

        // ! let -> math -> literal -> others
        if text == "let" {
            i = _let::parser(
                &mut program,
                data,
                input,
                i,
                &previous_text,
                &mut identifiers,
                &mut first_identifiers,
            );
        } else if text == "const" {
            i = _const::parser(
                &mut program,
                data,
                input,
                i,
                &previous_text,
                &mut identifiers,
                &mut first_identifiers,
            );
        } else if text.chars().next().unwrap_or('\0') == '\"' {
            program.push(Node::new(
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Literal {
                    literal: text.to_string(),
                    l_type: Types::Str,
                }),
                None,
                None,
                None,
            ));
        } else if is_digit(text.chars().next().unwrap_or('\0')) {
            program.push(Node::new(
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Literal {
                    literal: text.to_string(),
                    l_type: Types::I32, // * i32 is the default number
                }),
                None,
                None,
                None,
            ));
        } else if text == "include" || text == "return" || text == "erase" {
            i = include_n_return_n_erase::parser(&mut program, data, text);
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
                &mut first_identifiers,
            );
        } else if text == "while" {
            i = _while::parser(&mut program, data, &mut identifiers, &mut first_identifiers);
        } else if text == "for" {
            i = _for::parser(&mut program, data, &mut identifiers, &mut first_identifiers);
        } else if text == "struct" {
            i = _struct::parser(&mut program, data);
        } else if text == "enum" {
            i = _enum::parser(&mut program, data);
        } else if text == "fun" {
            i = fun::parser(&mut program, data, &mut identifiers, &mut first_identifiers);
        } else if has(
            &data.1,
            vec![
                "+", "-", "*", "/", "==", "=", ">", "<", "<=", ">=", "!=", "+=", "-=", "*=", "/=",
            ],
            Mode::OR,
        ) {
            i = math::parser(
                &mut program,
                text,
                data,
                input,
                i,
                &mut identifiers,
                &mut first_identifiers,
            );
        } else if text == "match" {
            i = _match::parser(&mut program, data, &mut identifiers, &mut first_identifiers);
        } else if has(&data.1, vec!["(", ")"], Mode::AND) {
            // also, it's a function call when there is no fun
            i = fun_call::parser(
                &mut program,
                text,
                input,
                i,
                &mut identifiers,
                &mut first_identifiers,
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
