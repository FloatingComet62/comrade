use crate::{
    compiler::compiler,
    exit,
    parser::{has, Mode, Parser, ParserData},
    Math, Node, NodeData, Operations,
};

use super::NodeInterferace;

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

pub struct MathManager {
    data: Vec<String>,
    cells: Vec<String>,
    is_math: bool,
}
impl MathManager {
    pub fn new(data: Vec<String>, cells: Vec<String>, is_math: bool) -> Self {
        Self {
            data,
            cells,
            is_math,
        }
    }
}
impl NodeInterferace<Math> for MathManager {
    fn check(&self, _text: String) -> bool {
        self.is_math && !has(&self.data, vec!["let", "const"], Mode::Or) && {
            // checking if the math found is inside a block
            // eg. fib(x-1)
            // it's a function call, not a math
            let mut unblocked_op = vec![];
            let mut blocked = 0;
            for cell in self.cells.iter() {
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
    }
    fn parser(
        &self,
        parser: Parser,
        program: &mut Vec<crate::Node>,
        data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
        text: &String,
        _previous_text: &String,
        _input: &Vec<String>,
        i: &mut usize,
        parser_data: &mut ParserData,
    ) {
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
        let mut raw_lhs = parser.get_till_token_or_block_and_math_block(operator, *i);
        let rhs = parser.get_till_token_or_block_and_math_block("EOL", raw_lhs.0);
        lhs.append(&mut raw_lhs.1);

        program.push(Node::new(
            NodeData::Math(Math {
                lhs: {
                    let mut parser = Parser::new(lhs, parser_data.clone());
                    parser.load();
                    parser.program
                },
                rhs: {
                    let mut parser = Parser::new(rhs.1, parser_data.clone());
                    parser.load();
                    parser.program
                },
                operation,
            }),
            0,
            0,
        ));
        *i = rhs.0 // skip to next and ignore the data
    }
    fn compiler(
        &self,
        data: Math,
        semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        let mut output = String::new();
        output += &compiler(&data.lhs, String::new(), false, false);
        output += match data.operation {
            Operations::ADD => "+",
            Operations::ADDEQT => "+=",
            Operations::DIV => "/",
            Operations::DIVEQT => "/=",
            Operations::EQGR => ">=",
            Operations::EQLT => "<=",
            Operations::MUL => "*",
            Operations::MULEQT => "*=",
            Operations::NEQ => "!=",
            Operations::NULL => exit("Unknown operation", None),
            Operations::SUB => "-",
            Operations::SUBEQT => "-=",
            Operations::GR => ">",
            Operations::LT => "<",
            Operations::EQT => "==",
            Operations::EQ => "=",
        };
        output += &compiler(&data.rhs, String::new(), false, false);
        if semi_colon_needed {
            output += ";";
        }
        Some(output)
    }
}
