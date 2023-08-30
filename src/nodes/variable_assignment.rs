use crate::{
    compiler::{compiler, type_to_c_type},
    exit,
    parser::{Parser, ParserData},
    type_from_str, Node, NodeData, Types, VariableAssignment,
};

use super::NodeInterferace;

fn val_getter(
    i_type: &String,
    raw_val: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    parser_data: &mut ParserData,
    iden: &Vec<String>,
) -> Vec<Node> {
    let i_type_type = type_from_str(&i_type);
    if !(i_type_type == Types::None) {
        let mut parser = Parser::new(raw_val.1, parser_data.clone());
        parser.load();
        return parser.program;
    }
    // it's not list
    if i_type.is_empty() {
        let mut parser = Parser::new(raw_val.1, parser_data.clone());
        parser.load();
        return parser.program;
    }
    let mut parser = Parser::new(raw_val.2, parser_data.clone());
    parser.load();
    let mut self_data = parser_data.struct_data.clone();
    self_data.retain(|x| x[0] == i_type.to_string()); // only 1 answer
    for (i, cell) in parser.program.iter().enumerate() {
        match &cell.data {
            NodeData::Literal(_) => {
                let mut item_iden = iden.clone();
                let raw_member = self_data[0].get(i + 1);
                // at self_data[0][0] is the struct name
                if let Some(member) = raw_member {
                    item_iden.push(member.clone());
                    parser_data.identifier.push(item_iden);
                } else {
                    exit(
                        &format!("Unknown values of struct {} were passed", self_data[0][0]),
                        None,
                    )
                }
            }
            _ => todo!(),
        }
    }
    return parser.program;
}

fn value(input: &mut VariableAssignment) -> String {
    if input.value.len() == 1 {
        return compiler(&input.value, String::new(), false, false);
    }
    let mut output = "{ ".to_string();
    for (i, item) in input.value.iter().enumerate() {
        output += &compiler(&vec![item.clone()], String::new(), false, true);
        if i != input.value.len() - 1 {
            output += ", ";
        }
    }
    output += " }";
    output
}

fn types(input: &VariableAssignment) -> (String, bool) {
    if input.value.len() == 1 {
        match &input.value[0].data {
            NodeData::Literal(l) => {
                let res = type_to_c_type(&l.l_type);
                return (res.0.to_string(), res.1);
            }
            _ => {
                let res = type_to_c_type(&input.type_data);
                return (res.0.to_string(), res.1);
            }
        };
    }
    if input.type_data == Types::None {
        let mut output = "struct ".to_string();
        output += &format!("{:?}", &input.type_data);
        return (output, false);
    }
    let res = type_to_c_type(&input.type_data);
    (res.0.to_string(), res.1)
}

pub struct VariableAssignmentManager {
    immutable: bool,
}
impl VariableAssignmentManager {
    pub fn new(immutable: bool) -> Self {
        Self { immutable }
    }
}
impl NodeInterferace<VariableAssignment> for VariableAssignmentManager {
    fn check(&self, text: String) -> bool {
        return text == "let" || text == "const";
    }
    fn parser(
        &self,
        parser: Parser,
        program: &mut Vec<crate::Node>,
        data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
        _text: &String,
        previous_text: &String,
        _input: &Vec<String>,
        i: &mut usize,
        mut parser_data: &mut ParserData,
    ) {
        let raw_iden = parser.get_till_token_or_block_and_math_block("=", *i);
        let mut iden = vec![];
        let mut i_type = String::new();
        let mut getting_iden = true;
        for item in raw_iden.1 {
            if item == "->" {
                getting_iden = false;
                continue;
            }
            if getting_iden {
                iden.push(item.clone());
                continue;
            }
            i_type = item;
        }
        let raw_val = parser.get_till_token_or_block_and_math_block("EOL", raw_iden.0);
        let val = val_getter(&i_type, raw_val, &mut parser_data, &iden.clone());
        let iden_str = iden.clone();

        parser_data.identifier.push(iden_str);
        program.push(Node::new(
            NodeData::VariableAssignment(VariableAssignment {
                identifier: iden,
                value: Box::new(val),
                immutability: self.immutable,
                publicity: previous_text == "public",
                type_data: type_from_str(&i_type),
            }),
            0,
            0,
        ));
        *i = data.0 // skip to next and ignore the data
    }
    fn compiler(
        &self,
        data: VariableAssignment,
        _semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        let mut output = String::new();
        let type_data = &types(&data);
        output += &type_data.0;
        output += " ";
        output += &data.identifier.join("_");
        if type_data.1 {
            output += "[]"
        }
        output += " = ";
        output += &value(&mut data.clone());
        output += ";";
        Some(output)
    }
}
