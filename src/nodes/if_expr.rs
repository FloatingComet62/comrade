use crate::{
    compiler::compiler,
    exit,
    parser::{Parser, ParserData},
    ConditionBlock, Node, NodeData,
};

use super::NodeInterferace;

/* Returns:
  0 - nothing
  1 - Is the last node an if?
  2 - Is the last node an else if?
*/
fn is_last_an_if(program: &Vec<Node>) -> u8 {
    let blank = Node::blank();
    let last_node = if program.is_empty() {
        &blank
    } else {
        program.last().unwrap_or(&blank)
    };
    match &last_node.data {
        NodeData::ConditionBlock(condition_node) => {
            if condition_node.keyword == "if" {
                return 1;
            }
            if condition_node.keyword == "else if" {
                return 2;
            }
            0
        }
        _ => todo!(),
    }
}

pub struct IfElseManager {}
impl IfElseManager {
    pub fn new() -> Self {
        Self {}
    }
}
impl NodeInterferace<ConditionBlock> for IfElseManager {
    fn check(&self, text: String) -> bool {
        text == "if" || text == "else"
    }
    fn parser(
        &self,
        _parser: Parser,
        program: &mut Vec<crate::Node>,
        data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
        text: &String,
        previous_text: &String,
        input: &Vec<String>,
        i: &mut usize,
        parser_data: &mut ParserData,
    ) {
        if text == "if" {
            if previous_text == "else" {
                let last_data = is_last_an_if(program);
                if last_data == 0 {
                    exit("Missing if part for else if", None);
                }
                program.push(Node::new(
                    NodeData::ConditionBlock(ConditionBlock {
                        keyword: "else if".to_string(),
                        parameters: {
                            let mut parser = Parser::new(data.1, parser_data.clone());
                            parser.load();
                            parser.program
                        },
                        nodes: {
                            let mut parser = Parser::new(data.2.clone(), parser_data.clone());
                            parser.load();
                            parser.program
                        },
                    }),
                    0,
                    0,
                ));
                *i = data.0;
                return;
            }
            program.push(Node::new(
                NodeData::ConditionBlock(ConditionBlock {
                    keyword: "if".to_string(),
                    parameters: {
                        let mut parser = Parser::new(data.1, parser_data.clone());
                        parser.load();
                        parser.program
                    },
                    nodes: {
                        let mut parser = Parser::new(data.2.clone(), parser_data.clone());
                        parser.load();
                        parser.program
                    },
                }),
                0,
                0,
            ));
        }
        if text == "else" {
            // if next token is "if", then skip this, else if is handled above
            let blank = String::new();
            let next_token = input.get(i.clone() + 1).unwrap_or(&blank);
            if next_token == "if" {
                return;
            }

            let last_node = is_last_an_if(program);
            if last_node == 0 {
                exit("Missing if part for else", None);
            }
            program.push(Node::new(
                NodeData::ConditionBlock(ConditionBlock {
                    keyword: "else".to_string(),
                    parameters: vec![],
                    nodes: {
                        let mut parser = Parser::new(data.2.clone(), parser_data.clone());
                        parser.load();
                        parser.program
                    },
                }),
                0,
                0,
            ));
        }
        *i = data.0 // skip to next and ignore the data
    }
    fn compiler(
        &self,
        data: ConditionBlock,
        _semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        let mut output = String::new();
        if data.keyword == "if" {
            output += "if(";
            output += &compiler(&data.parameters, String::new(), false, true);
            output += "){";
            output += &compiler(&data.nodes, String::new(), true, false);
            output += "}";
        }
        if data.keyword == "else if" {
            output += "else if(";
            output += &compiler(&data.parameters, String::new(), false, true);
            output += "){";
            output += &compiler(&data.nodes, String::new(), true, false);
            output += "}";
        }
        if data.keyword == "else" {
            output += "else {";
            output += &compiler(&data.nodes, String::new(), true, false);
            output += "}";
        }
        if data.keyword == "while" {
            output += "while (";
            output += &compiler(&data.parameters, String::new(), false, true);
            output += ") {";
            output += &compiler(&data.nodes, String::new(), true, false);
            output += "}";
        }
        Some(output)
    }
}
