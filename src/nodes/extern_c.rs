use crate::{
    parser::{Parser, ParserData},
    ExternC, Node, NodeData,
};

use super::NodeInterferace;

pub struct ExternCManager {}
impl ExternCManager {
    pub fn new() -> Self {
        Self {}
    }
}
impl NodeInterferace<ExternC> for ExternCManager {
    fn check(&self, text: String) -> bool {
        text == "externC"
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
        _parser_data: &mut ParserData,
    ) {
        program.push(Node::new(
            NodeData::ExternC(ExternC {
                block: format!("\n{}\n", data.1[0].clone()),
            }),
            0,
            0,
        ));
        *i = data.0 // skip to next and ignore the data
    }
    fn compiler(
        &self,
        data: ExternC,
        _semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        Some(data.block)
    }
}
