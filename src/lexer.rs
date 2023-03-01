use crate::Types;

#[derive(Debug)]
pub enum NodeTypes {
    STATEMENT,
    FUNCTION,
    FUNCTIONCALL,
}
pub struct Argument {
    identifier: Vec<String>,
    a_type: Types,
}
pub struct Literal {
    literal: String,
    l_type: Types,
}
pub struct ArgumentLiteral {
    argument: Option<Argument>,
    literal: Option<Literal>,
}
pub struct Statement {
    action: String,
    parameters: Vec<String>,
}
pub struct Function {
    identifier: Vec<String>,
    return_type: Types,
    arguments: Vec<Argument>,
    nodes: Vec<Node>,
}
pub struct FunctionCall {
    identifier: Vec<String>,
    arguments: Vec<ArgumentLiteral>,
}
pub struct NodeData {
    statement: Option<Statement>,
    function: Option<Function>,
    function_call: Option<FunctionCall>,
}
pub struct Node {
    n_type: NodeTypes,
    data: NodeData,
}

pub struct Lexer {
    splitted_text: Vec<String>,
    program: Vec<Node>,
    keywords: Vec<String>,
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
    pub fn load(self: &Lexer) {
        for text in &self.splitted_text {
            if self.keywords.contains(&text) {}
        }
    }
}
