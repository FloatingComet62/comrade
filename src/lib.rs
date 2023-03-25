use std::fmt::Debug;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::process;

pub mod compiler;
pub mod lexer;
pub mod parser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Types {
    U4 = 1,
    U4List = 33,
    U8 = 2,
    U8List = 34,
    U16 = 3,
    U16List = 35,
    U32 = 4,
    U32List = 36,
    U64 = 5,
    U64List = 37,
    U128 = 6,
    U128List = 38,
    I4 = 7,
    I4List = 39,
    I8 = 8,
    I8List = 40,
    I16 = 9,
    I16List = 41,
    I32 = 10,
    I32List = 42,
    I64 = 11,
    I64List = 43,
    I128 = 12,
    I128List = 44,
    F4 = 13,
    F4List = 45,
    F8 = 14,
    F8List = 46,
    F16 = 15,
    F16List = 47,
    F32 = 16,
    F32List = 48,
    F64 = 17,
    F64List = 49,
    F128 = 18,
    F128List = 50,
    Str = 19,
    StrList = 51,
    Bool = 21,
    BoolList = 52,
    None = 22,

    Type = 23,
}

pub fn read_file(path: &String) -> String {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(data) => data,
        Err(e) => exit(
            &format!("Unable to read {}\nError Trace:\n{}", path, e),
            None,
        ),
    }
}

pub fn write_file(path: &str, code: String) -> Result<(), std::io::Error> {
    fs::write(path, code)
}

pub fn exit(msg: &str, code: Option<i32>) -> ! {
    println!("{}", msg);
    process::exit(code.unwrap_or(1));
}

pub fn type_from_str(string: &str) -> Types {
    match string {
        "u4" => Types::U4,
        "u8" => Types::U8,
        "u16" => Types::U16,
        "u32" => Types::U32,
        "u64" => Types::U64,
        "u128" => Types::U128,
        "i4" => Types::I4,
        "i8" => Types::I8,
        "i16" => Types::I16,
        "i32" => Types::I32,
        "i64" => Types::I64,
        "i128" => Types::I128,
        "f4" => Types::F4,
        "f8" => Types::F8,
        "f16" => Types::F16,
        "f32" => Types::F32,
        "f64" => Types::F64,
        "f128" => Types::F128,
        "str" => Types::Str,
        "bool" => Types::Bool,
        "u4[]" => Types::U4List,
        "u8[]" => Types::U8List,
        "u16[]" => Types::U16List,
        "u32[]" => Types::U32List,
        "u64[]" => Types::U64List,
        "u128[]" => Types::U128List,
        "i4[]" => Types::I4List,
        "i8[]" => Types::I8List,
        "i16[]" => Types::I16List,
        "i32[]" => Types::I32List,
        "i64[]" => Types::I64List,
        "i128[]" => Types::I128List,
        "f4[]" => Types::F4List,
        "f8[]" => Types::F8List,
        "f16[]" => Types::F16List,
        "f32[]" => Types::F32List,
        "f64[]" => Types::F64List,
        "f128[]" => Types::F128List,
        "str[]" => Types::StrList,
        "bool[]" => Types::BoolList,
        "type" => Types::Type,
        _ => Types::None,
    }
}

pub fn str_list_to_string_list(input: Vec<&str>) -> Vec<String> {
    let mut output = vec![];

    for item in input {
        output.push(item.to_string());
    }

    output
}

#[macro_export]
macro_rules! node {
    ($x: ident , $y: expr) => {{
        let mut n = Node::blank();
        n.$x = Some($y);
        n
    }};
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operations {
    NULL,

    /// "+"
    /// addition
    ADD,
    /// "-"
    /// subtraction
    SUB,
    /// "*"
    /// multiplication
    MUL,
    /// "/"
    /// division
    DIV,
    /// ">="
    /// equal or greater thsn
    EQGR,
    /// "<="
    /// equal or less than
    EQLT,
    /// ">"
    /// greater than
    GR,
    /// "<"
    /// less than
    LT,
    /// "!="
    /// not equal
    NEQ,
    /// "=="
    /// equate to rhs
    EQT,
    /// "+="
    /// add rhs to lhs
    ADDEQT,
    /// "-="
    /// subtract rhs to lhs
    SUBEQT,
    /// "*="
    /// multiply rhs to lhs"
    MULEQT,
    /// "/="
    /// divide rhs to lhs
    DIVEQT,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub identifier: String,
    pub a_type: Types,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub literal: String,
    pub l_type: Types,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub action: String,
    pub parameters: Vec<Node>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub identifier: Vec<String>,
    pub return_type: Types,
    pub arguments: Vec<Argument>,
    pub nodes: Vec<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub identifier: Vec<String>,
    pub arguments: Vec<Vec<Node>>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct VariableAssignment {
    pub identifier: Vec<String>,
    pub value: Box<Vec<Node>>,
    pub immutability: bool,
    pub publicity: bool,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub expr: Vec<String>, // maybe node? idk
}
#[derive(Debug, PartialEq, Clone)]
pub struct ConditionBlock {
    pub keyword: String,
    pub parameters: Vec<Node>,
    pub nodes: Vec<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct MatchCase {
    pub case: Vec<Node>,
    pub block: Vec<Node>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Match {
    pub condition: Vec<Node>,
    pub block: Vec<MatchCase>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct StructMember {
    pub identifier: Vec<String>,
    pub t_mem: Types,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Struct {
    pub identifier: Vec<String>,
    pub members: Vec<StructMember>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Enum {
    pub identifier: Vec<String>,
    pub members: Vec<String>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Math {
    pub lhs: Vec<Node>,
    pub rhs: Vec<Node>,
    pub operation: Operations,
}
#[derive(Debug, PartialEq, Clone)]
pub struct ExternC {
    pub block: String,
}
#[derive(PartialEq, Clone)]
pub struct NodeData {
    pub statement: Option<Statement>,
    pub function: Option<Function>,
    pub function_call: Option<FunctionCall>,
    pub variable_assignment: Option<VariableAssignment>,
    pub expression: Option<Expression>,
    pub condition_block: Option<ConditionBlock>,
    pub _match: Option<Match>,
    pub literal: Option<Literal>,
    pub math: Option<Math>,
    pub _struct: Option<Struct>,
    pub _enum: Option<Enum>,
    pub extern_c: Option<ExternC>,
}
impl Debug for NodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! check {
            ($x: expr) => {
                if let Some(x) = $x {
                    return f.write_str(&format!("{:?}", x));
                }
            };
        }

        check!(&self.statement);
        check!(&self.function);
        check!(&self.function_call);
        check!(&self.variable_assignment);
        check!(&self.expression);
        check!(&self.condition_block);
        check!(&self._match);
        check!(&self.literal);
        check!(&self.math);
        check!(&self._struct);
        check!(&self._enum);
        check!(&self.extern_c);

        f.write_str("{}")
    }
}
// todo: if by the end of the parser, all node has is "data", just make Node NodeData
#[derive(PartialEq, Clone)]
pub struct Node {
    pub data: NodeData,
}

impl Deref for Node {
    type Target = NodeData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
