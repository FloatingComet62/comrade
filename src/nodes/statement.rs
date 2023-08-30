use crate::{
    compiler::compiler,
    lexer::Lexer,
    parser::{Parser, ParserData, KEYWORDS, LIB},
    read_file, Node, NodeData, Statement, FILE_EXTENSION,
};

use super::NodeInterferace;

fn param_to_path(param: &[Node]) -> String {
    let mut path = String::new();
    let param_vec = match &param[0].data {
        NodeData::Expression(e) => &e.expr,
        _ => todo!(),
    };
    for i in 0..param_vec.len() {
        path += &param_vec[i];
        if i != param_vec.len() - 1 {
            path += "/";
        }
    }
    path += FILE_EXTENSION;
    path
}
fn param_to_iden_vec(params: &[Node], identifier: &[String]) -> Vec<String> {
    let mut output = vec![];
    let blank_str = &String::new();
    let binding = Node::blank();
    let expr = match &params.last().unwrap_or(&binding).data {
        NodeData::Expression(e) => &e.expr,
        _ => todo!(),
    };
    output.append(&mut vec![
        expr.last().unwrap_or(blank_str).to_string(),
        "_".to_string(),
    ]);
    output.append(&mut identifier.to_owned());
    output
}

pub struct StatementManager {}
impl StatementManager {
    pub fn new() -> Self {
        Self {}
    }
}
impl NodeInterferace<Statement> for StatementManager {
    fn check(&self, text: String) -> bool {
        text == "include" || text == "return" || text == "erase"
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
        parser_data: &mut ParserData,
    ) {
        let mut params = data.1.clone();
        params.retain(|x| x != "->");

        let mut parser = Parser::new(params.clone(), parser_data.clone());
        parser.load();

        program.push(Node::new(
            NodeData::Statement(Statement {
                action: text.to_string(),
                parameters: parser.program,
            }),
            0,
            0,
        ));
        *i = data.0 // skip to next and ignore the data
    }
    fn compiler(
        &self,
        data: Statement,
        _semi_colon_needed: bool,
        _is_inside_function_call: bool,
    ) -> Option<String> {
        let mut output = String::new();
        if data.action == "include" {
            let lib = param_to_path(&data.parameters);
            let lexer = Lexer::new(read_file(&lib));
            let mut lib_tokens_to_parse = vec![];
            let lib_tokens = {
                let parser_data = ParserData {
                    identifier: {
                        let mut output = vec![];
                        for keywords in KEYWORDS.iter() {
                            let item: Vec<String> =
                                keywords.iter().map(|x| x.to_string()).collect();
                            output.push(item);
                        }
                        for keywords in LIB.iter() {
                            let item: Vec<String> =
                                keywords.iter().map(|x| x.to_string()).collect();
                            output.push(item);
                        }
                        output
                    },
                    enum_values: vec![],
                    struct_data: vec![],
                };
                let mut parser = Parser::new(lexer.token_splitter(), parser_data);
                parser.load();
                parser.program
            };
            for i in 0..lib_tokens.len() {
                let raw_lib_token = &lib_tokens.get(i);
                if raw_lib_token.is_none() {
                    continue;
                }
                let lib_token = &mut raw_lib_token.unwrap().clone();
                match &mut lib_token.data {
                    NodeData::Function(func) => {
                        if func.identifier == vec!["__init__"] {
                            // __init__ is embedded directly and the function is removed
                            output += &compiler(&func.nodes, String::new(), true, false);
                        } else {
                            // normal function, add the path->function_name
                            //todo maybe with variables
                            let new_identifier =
                                param_to_iden_vec(&data.parameters, &func.identifier);
                            func.identifier = new_identifier.clone();

                            lib_tokens_to_parse.push(lib_token.clone());
                        }
                    }
                    _ => todo!(),
                }
            }
            output += &compiler(&lib_tokens_to_parse, String::new(), true, false);
        } else {
            output += &data.action;
            output += " ";
            output += &compiler(&data.parameters, String::new(), true, false);
            output += ";";
        }
        Some(output)
    }
}
