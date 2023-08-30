use crate::{
    compiler::compiler,
    parser::{Parser, ParserData},
    Expression, Match, MatchCase, Node, NodeData,
};

use super::NodeInterferace;

pub fn get_match_case(
    input: &Vec<String>,
    parser_data: ParserData,
    i: usize,
) -> (usize, Option<MatchCase>) {
    let parser = Parser::new(input.clone(), parser_data);
    let case_data = parser.get_till_token_or_block_and_math_block("=>", i);
    let expr_data = parser.get_till_token_or_block_and_math_block("EOL", case_data.0);
    let case_target = if case_data.3 {
        if case_data.2.is_empty() {
            return (case_data.0, None);
        }
        case_data.2
    } else {
        if case_data.1.is_empty() {
            return (case_data.0, None);
        }
        case_data.1
    };
    let expr_target = if expr_data.3 {
        if expr_data.2.is_empty() {
            return (case_data.0, None);
        }
        expr_data.2
    } else {
        if expr_data.1.is_empty() {
            return (case_data.0, None);
        }
        expr_data.1
    };
    (
        expr_data.0,
        Some(MatchCase {
            block: {
                let mut parser = Parser::new(expr_target, parser.parser_data.clone());
                parser.load();
                parser.program
            },
            case: {
                let mut parser = Parser::new(case_target, parser.parser_data.clone());
                parser.load();
                parser.program
            },
        }),
    )
}

pub struct MatchManager {}
impl MatchManager {
    pub fn new() -> Self {
        Self {}
    }
}
impl NodeInterferace<Match> for MatchManager {
    fn check(&self, text: String) -> bool {
        text == "match"
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
        let mut j = 0;
        println!("{:#?}", data);
        while j < data.2.len() {
            let x = get_match_case(&data.2, parser.parser_data.clone(), j);
            if let Some(y) = x.1 {
                block.push(y);
            }
            j = x.0;
        }

        program.push(Node::new(
            NodeData::Match(Match {
                condition: {
                    let mut parser = Parser::new(data.1, parser_data.clone());
                    parser.load();
                    parser.program
                },
                block,
            }),
            0,
            0,
        ));
        *i = data.0 // skip to next and ignore the data
    }
    fn compiler(
        &self,
        data: Match,
        semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        let mut output = String::new();
        let to_eval = &compiler(&data.condition, String::new(), semi_colon_needed, false);
        for (i, case) in data.block.iter().enumerate() {
            if i == 0 {
                output += "if(";
                output += to_eval;
                output += " == ";
                output += &compiler(&case.case.clone(), String::new(), semi_colon_needed, false);
                output += ") {";
                output += &compiler(&case.block.clone(), String::new(), semi_colon_needed, false);
                output += "}";
                continue;
            }
            if case.case
                == vec![Node::new(
                    NodeData::Expression(Expression {
                        expr: vec!["default".to_string()],
                    }),
                    0,
                    0,
                )]
            {
                output += "else {";
                output += &compiler(&case.block.clone(), String::new(), semi_colon_needed, false);
                output += "}";
                continue;
            }
            output += "else if(";
            output += to_eval;
            output += " == ";
            output += &compiler(&case.case.clone(), String::new(), semi_colon_needed, false);
            output += ") {";
            output += &compiler(&case.block.clone(), String::new(), semi_colon_needed, false);
            output += "}";
        }
        Some(output)
    }
}
