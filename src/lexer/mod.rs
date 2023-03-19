use crate::Types;
use std::{fmt::Debug, ops::Deref};

mod _let;
mod _match;
mod _while;
mod fun;
mod fun_call;
mod include_n_return;

#[derive(Debug)]
pub enum NodeTypes {
    STATEMENT,
    FUNCTION,
    FUNCTIONCALL,
    VARIABLEASSIGNMENT,
    EXPRESSION,
}
#[derive(Debug, Clone)]
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
pub struct Statement {
    pub action: String,
    pub parameters: Vec<String>, //todo Vec<Node>
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
    pub arguments: Vec<Vec<Node>>,
}
#[derive(Debug)]
pub struct VariableAssignment {
    pub identifier: Vec<String>,
    pub value: Box<Node>,
}
#[derive(Debug)]
pub struct Expression {
    pub expr: Vec<String>,
}
#[derive(Debug)]
pub struct ConditionBlock {
    pub keyword: String,
    pub parameters: Vec<String>, //todo Vec<Node>
    pub nodes: Vec<Node>,
}
#[derive(Debug)]
pub struct MatchCase {
    pub case: Vec<String>, // maybe something else, eg. ["addition", or, "add", or "a"]
    pub block: Vec<Node>,
}
#[derive(Debug)]
pub struct Match {
    pub condition: Vec<String>,
    pub block: Vec<MatchCase>,
}
pub struct NodeData {
    pub statement: Option<Statement>,
    pub function: Option<Function>,
    pub function_call: Option<FunctionCall>,
    pub variable_assignment: Option<VariableAssignment>,
    pub expression: Option<Expression>,
    pub condition_block: Option<ConditionBlock>,
    pub _match: Option<Match>,
    pub literal: Option<Literal>,
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
        if let Some(variable_assignment) = &self.variable_assignment {
            return f.write_str(&format!("{:?}", variable_assignment));
        }
        if let Some(expression) = &self.expression {
            return f.write_str(&format!("{:?}", expression));
        }
        if let Some(condition_block) = &self.condition_block {
            return f.write_str(&format!("{:?}", condition_block));
        }
        if let Some(_match) = &self._match {
            return f.write_str(&format!("{:?}", _match));
        }
        if let Some(literal) = &self.literal {
            return f.write_str(&format!("{:?}", literal));
        }

        f.write_str("{}")
    }
}
// todo: if by the end of the parser, all node has is "data", just make Node NodeData
pub struct Node {
    pub data: NodeData,
}
impl Deref for Node {
    type Target = NodeData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
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
        variable_assignment: Option<VariableAssignment>,
        expression: Option<Expression>,
        condition_block: Option<ConditionBlock>,
        _match: Option<Match>,
        literal: Option<Literal>,
    ) -> Self {
        Self {
            data: NodeData {
                statement,
                function,
                function_call,
                variable_assignment,
                expression,
                condition_block,
                _match,
                literal,
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
    content till token
    block found
*/
pub fn get_till_token_or_block(
    token: &str,
    text: &Vec<String>,
    i: usize,
) -> (usize, Vec<String>, Vec<String>, bool) {
    let mut output: Vec<String> = vec![];
    let mut block: Vec<String> = vec![];
    let mut got_block = false;
    let mut getting_block = false;
    let mut j = i + 1;
    while j < text.len() {
        let text = &text[j];
        // ! must be first
        if text == "{" {
            getting_block = true;
        }
        if text == token && !getting_block {
            break;
        }
        if getting_block {
            block.push(text.to_string());
        } else {
            output.push(text.to_string());
        }
        // ! must be last
        if text == "}" {
            got_block = true;
            getting_block = false;
        }

        j += 1;
    }
    return (j, output, block, got_block);
}
pub fn load(input: &Vec<String>) -> Vec<Node> {
    let mut program = vec![];
    let mut i = 0;
    while i < input.len() {
        let data = get_till_token_or_block("EOL", input, i);
        let text = &input[i];

        // skip EOLs
        if text == "EOL" {
            i += 1;
            continue;
        }

        if text == "include" || text == "return" {
            i = include_n_return::parser(&mut program, text, input, i);
        } else if text == "let" {
            i = _let::parser(&mut program, input, i);
        } else if text == "while" {
            i = _while::parser(&mut program, input, i);
        } else if text == "match" {
            i = _match::parser(&mut program, input, i);
        } else if text == "fun" {
            i = fun::parser(&mut program, data);
            // else if, because it's a function call when there is no "fun"
            // quite literally
        } else if data.1.contains(&"(".to_string()) && data.1.contains(&")".to_string()) {
            i = fun_call::parser(&mut program, data, i);
        }
        i += 1;
    }

    program
}
