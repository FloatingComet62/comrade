use crate::Types;
use std::fmt::Debug;

mod fun;
mod include_n_return;

#[derive(Debug)]
pub enum NodeTypes {
    STATEMENT,
    FUNCTION,
    FUNCTIONCALL,
}
#[derive(Debug)]
pub struct Argument {
    pub identifier: String,
    pub a_type: Types,
}
#[derive(Debug)]
pub struct Literal {
    pub literal: String,
    pub l_type: Types,
}
#[derive(Debug)]
pub struct ArgumentLiteral {
    pub argument: Option<Argument>,
    pub literal: Option<Literal>,
}
#[derive(Debug)]
pub struct Statement {
    pub action: String,
    pub parameters: Vec<String>,
}
#[derive(Debug)]
pub struct Function {
    pub identifier: Vec<String>,
    pub return_type: Types,
    pub arguments: Vec<Argument>,
    pub nodes: Vec<Node>,
}
#[derive(Debug)]
pub struct FunctionCall {
    pub identifier: Vec<String>,
    pub arguments: Vec<ArgumentLiteral>,
}
pub struct NodeData {
    pub statement: Option<Statement>,
    pub function: Option<Function>,
    pub function_call: Option<FunctionCall>,
}
impl Debug for NodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(statement) = &self.statement {
            return f.write_str(&format!("{:?}", statement));
        }
        if let Some(function) = &self.function {
            return f.write_str(&format!("{:?}", function));
        }
        if let Some(function_call) = &self.function_call {
            return f.write_str(&format!("{:?}", function_call));
        }

        f.write_str("{}")
    }
}
pub struct Node {
    pub data: NodeData,
}
impl Debug for Node {
    // don't display n_type
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.data))
    }
}
impl Node {
    pub fn new(
        statement: Option<Statement>,
        function: Option<Function>,
        function_call: Option<FunctionCall>,
    ) -> Self {
        Self {
            data: NodeData {
                statement,
                function,
                function_call,
            },
        }
    }
}

pub struct Lexer {
    pub splitted_text: Vec<String>,
    pub program: Vec<Node>,
    pub keywords: Vec<String>,
}
impl Lexer {
    pub fn new(splitted_text: Vec<String>) -> Self {
        Self {
            splitted_text,
            program: vec![],
            keywords: vec![
                String::from("include"),
                String::from("fun"),
                String::from("return"),
            ],
        }
    }
}

/*
    Returns:
    new index
    content till eol
    block found
*/
pub fn get_till_eol_or_block(text: &Vec<String>, i: usize) -> (usize, Vec<String>, Vec<String>) {
    let mut output: Vec<String> = vec![];
    let mut block: Vec<String> = vec![];
    let mut getting_block = false;
    let mut j = i + 1;
    while j < text.len() {
        let text = &text[j];
        // ! must be first
        if text == "{" {
            getting_block = true;
        }
        if text == "EOL" && !getting_block {
            break;
        }
        if getting_block {
            block.push(text.to_string());
        } else {
            output.push(text.to_string());
        }
        // ! must be last
        if text == "}" {
            getting_block = false;
        }

        j += 1;
    }
    return (j, output, block);
}
pub fn load(input: &Vec<String>) -> Vec<Node> {
    let mut program = vec![];
    let mut i = 0;
    while i < input.len() {
        let text = &input[i];
        if text == "include" || text == "return" {
            i = include_n_return::parser(&mut program, text, input, i);
        } else if text == "fun" {
            i = fun::parser(&mut program, input, i);
        }
        i += 1;
    }

    program
}
