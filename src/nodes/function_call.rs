use crate::{
    compiler::compiler,
    parser::{has, Mode, Parser, ParserData},
    FunctionCall, Node, NodeData,
};

use super::NodeInterferace;

pub struct FunctionCallManager {
    data: Vec<String>,
}
impl FunctionCallManager {
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }
}
impl NodeInterferace<FunctionCall> for FunctionCallManager {
    fn check(&self, _text: String) -> bool {
        has(&self.data, vec!["(", ")"], Mode::And)
    }
    fn parser(
        &self,
        parser: Parser,
        program: &mut Vec<crate::Node>,
        _data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
        text: &String,
        _previous_text: &String,
        _input: &Vec<String>,
        i: &mut usize,
        parser_data: &mut ParserData,
    ) {
        let mut identifier = vec![text.to_string()];
        let mut raw_identifier = parser.get_till_token_or_block_and_math_block("(", *i);

        identifier.append(&mut raw_identifier.1);

        let mut raw_args = vec![];
        let mut raw_raw_args =
            parser.get_till_token_or_block_and_math_block(")", raw_identifier.0 - 1);

        // basically, join the block you found with the main content
        raw_args.append(&mut raw_raw_args.1);
        raw_args.append(&mut raw_raw_args.2);

        // Raw_args: ["(", "5"]
        // if there is only 1 "(", then it's fine
        // but if there is more, than the missing ")" at the end messes up the program
        // eg. ["(", "fib", "(", "5" , ")"]
        let mut brack_count = 0;
        for cell in raw_args.iter() {
            if cell == "(" {
                brack_count += 1;
            }
        }

        if brack_count > 1 {
            raw_args.push(")".to_string());
        }

        let mut args: Vec<Vec<Node>> = vec![];
        let mut arg: Vec<String> = vec![];
        for item in &raw_args {
            if item == "," {
                if !arg.is_empty() {
                    arg.push("EOL".to_string());
                    args.push({
                        let mut parser = Parser::new(arg, parser_data.clone());
                        parser.load();
                        parser.program
                    });
                }
                arg = vec![];
                continue;
            }
            arg.push(item.to_string());
        }
        if !arg.is_empty() {
            arg.push("EOL".to_string());
            args.push({
                let mut parser = Parser::new(arg, parser_data.clone());
                parser.load();
                parser.program
            });
        }
        program.push(Node::new(
            NodeData::FunctionCall(FunctionCall {
                identifier,
                arguments: args,
            }),
            0,
            0,
        ));
        *i = raw_raw_args.0
    }
    fn compiler(
        &self,
        data: FunctionCall,
        semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        let mut output = String::new();
        for i in 0..data.identifier.len() {
            let item = &data.identifier[i];
            if item == "->" {
                output += "_";
            } else {
                output += item;
            }
            if i != data.identifier.len() - 1 {
                output += "_";
            }
        }
        output += "(";
        for item in &data.arguments {
            output += &compiler(&item.clone(), String::new(), false, true);
        }
        output += ")";
        if semi_colon_needed {
            output += ";";
        }
        Some(output)
    }
}
