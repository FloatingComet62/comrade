use crate::{
    compiler::type_to_c_type,
    errors::{send_error, Errors},
    parser::{Parser, ParserData},
    type_from_str, Node, NodeData, Struct, StructMember, Types,
};

use super::NodeInterferace;

pub fn get_struct_member(
    input: &Vec<String>,
    parser_data: ParserData,
    i: usize,
) -> (usize, Option<StructMember>) {
    let parser = Parser::new(input.clone(), parser_data);
    let mut case_data = parser.get_till_token_or_block_and_math_block("->", i);
    let mut type_data = parser.get_till_token_or_block_and_math_block("EOL", case_data.0);

    case_data.1.retain(|x| x != "EOL");
    type_data.1.retain(|x| x != "EOL");

    if type_data.1.is_empty() {
        return (type_data.0, None);
    }

    let t_mem = type_from_str(&type_data.1[0]);

    if t_mem == Types::None {
        send_error(
            Errors::UNDEFINEDIDENTIFIER,
            format!("Expected Type, found {}", &type_data.1[0]),
            0,
            0,
        );
    }

    (
        type_data.0,
        Some(StructMember {
            identifier: case_data.1,
            t_mem,
        }),
    )
}

fn types(input: &StructMember) -> (String, bool) {
    let res = type_to_c_type(&input.t_mem);
    (res.0.to_string(), res.1)
}

pub struct StructManager {}
impl StructManager {
    pub fn new() -> Self {
        Self {}
    }
}
impl NodeInterferace<Struct> for StructManager {
    fn check(&self, text: String) -> bool {
        text == "struct"
    }
    fn parser(
        &self,
        parser: Parser,
        program: &mut Vec<crate::Node>,
        data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
        _text: &String,
        _previous_text: &String,
        _input: &Vec<String>,
        i: &mut usize,
        parser_data: &mut ParserData,
    ) {
        let mut block = vec![];
        let mut self_data = vec![data.1.join("_")];
        let mut j = 0;
        while j < data.2.len() {
            let x = get_struct_member(&data.2, parser.parser_data.clone(), j);
            if let Some(y) = x.1 {
                self_data.push(y.identifier.join("_"));
                block.push(y);
            }
            j = x.0;
        }

        parser_data.struct_data.push(self_data);

        program.push(Node::new(
            NodeData::Struct(Struct {
                identifier: data.1,
                members: block,
            }),
            0,
            0,
        ));
        *i = data.0 // skip to next and ignore the data
    }
    fn compiler(
        &self,
        data: Struct,
        _semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        let mut output = "struct ".to_string();
        output += &data.identifier.join("_");
        output += "{";
        for item in data.members.iter() {
            let type_data = &types(item);
            output += &type_data.0;
            output += " ";
            output += &item.identifier.join("_");
            if type_data.1 {
                output += "[]"
            }
            output += ";";
        }
        output += "}";
        output += ";";
        Some(output)
    }
}
