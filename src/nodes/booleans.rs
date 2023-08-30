use crate::{
    parser::{Parser, ParserData},
    Literal, Node, NodeData, Types,
};

use super::NodeInterferace;

pub struct BooleanManager {}
impl BooleanManager {
    pub fn new() -> Self {
        Self {}
    }
}
impl NodeInterferace<Literal> for BooleanManager {
    fn check(&self, text: String) -> bool {
        text == "true" || text == "false"
    }
    fn parser(
        &self,
        _parser: Parser,
        program: &mut Vec<crate::Node>,
        data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
        text: &String,
        _previous_text: &String,
        _input: &Vec<String>,
        i: &mut usize,
        _parser_data: &mut ParserData,
    ) {
        program.push(Node::new(
            NodeData::Literal(Literal {
                literal: text.to_string(),
                l_type: Types::Bool,
            }),
            0,
            0,
        ));
        *i = data.0 // skip to next and ignore the data
    }
    fn compiler(
        &self,
        _data: Literal,
        _semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        None
    }
}
