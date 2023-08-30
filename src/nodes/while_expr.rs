use crate::{
    parser::{Parser, ParserData},
    ConditionBlock, Node, NodeData,
};

use super::NodeInterferace;

pub struct WhileManager {}
impl WhileManager {
    pub fn new() -> Self {
        Self {}
    }
}
impl NodeInterferace<ConditionBlock> for WhileManager {
    fn check(&self, text: String) -> bool {
        text == "while"
    }
    fn parser(
        &self,
        _parser: Parser,
        program: &mut Vec<crate::Node>,
        data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
        _text: &String,
        _previous_text: &String,
        _input: &Vec<String>,
        i: &mut usize,
        parser_data: &mut ParserData,
    ) {
        program.push(Node::new(
            NodeData::ConditionBlock(ConditionBlock {
                keyword: "while".to_string(),
                parameters: {
                    let mut parser = Parser::new(data.1, parser_data.clone());
                    parser.load();
                    parser.program
                },
                nodes: {
                    let mut parser = Parser::new(data.2, parser_data.clone());
                    parser.load();
                    parser.program
                },
            }),
            0,
            0,
        ));
        *i = data.0 // skip to next and ignore the data
    }
    fn compiler(
        &self,
        _data: ConditionBlock,
        _semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        None
    }
}
