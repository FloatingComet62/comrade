use crate::{
    parser::{Parser, ParserData},
    Enum, Node, NodeData,
};

use super::NodeInterferace;

pub struct EnumManager {}
impl EnumManager {
    pub fn new() -> Self {
        Self {}
    }
}
impl NodeInterferace<Enum> for EnumManager {
    fn check(&self, text: String) -> bool {
        text == "enum"
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
        let mut members = data.2.clone();
        members.retain(|x| !vec!["EOL", ",", "{", "}"].contains(&x.as_str()));

        for member in members.iter() {
            parser_data
                .enum_values
                .push(vec![data.1.join("_"), member.to_string()]);
        }
        program.push(Node::new(
            NodeData::Enum(Enum {
                identifier: data.1,
                members,
            }),
            0,
            0,
        ));
        *i = data.0
    }
    fn compiler(
        &self,
        data: Enum,
        _semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        let mut output = String::new();
        let name = data.identifier.join("_");
        output += "enum ";
        output += &name;
        output += " {";
        for item in data.members.iter() {
            output += &name; // enum ENUM { ITEM } => enum ENUM { ENUM_ITEM } for uniqueness among different enums
            output += "_";
            output += item;
            output += ", ";
        }
        output += "};";
        Some(output)
    }
}
