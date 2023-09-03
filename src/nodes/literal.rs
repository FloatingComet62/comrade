use crate::{
    parser::{Parser, ParserData},
    Literal, Node, NodeData, Types,
};

use super::NodeInterferace;

pub struct LiteralManager {
    pub l_types: Types,
}
impl LiteralManager {
    pub fn new(l_types: Types) -> Self {
        Self { l_types }
    }
}
impl NodeInterferace<Literal> for LiteralManager {
    fn check(&self, _text: String) -> bool {
        self.l_types != Types::None
    }
    fn parser(
        &self,
        _parser: Parser,
        program: &mut Vec<crate::Node>,
        _data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
        text: &String,
        _previous_text: &String,
        _input: &Vec<String>,
        _i: &mut usize,
        _parser_data: &mut ParserData,
    ) {
        program.push(Node::new(
            NodeData::Literal(Literal {
                literal: text.to_string(),
                l_type: self.l_types,
            }),
            0,
            0,
        ));
    }
    fn compiler(
        &self,
        data: Literal,
        _semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        if data.literal.contains('_') {
            let enum_vals: Vec<&str> = data.literal.split('_').collect();
            return Some(enum_vals[1].to_string());
        } else {
            return Some(data.literal);
        }
    }
}
