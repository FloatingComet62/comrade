use std::fmt::Debug;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::process;

pub mod compiler;
pub mod errors;
pub mod lexer;
pub mod parser;
pub mod type_checker;

pub const FILE_EXTENSION: &str = ".cmr";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Object = 24,
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

#[derive(Debug, PartialEq, Eq, Clone)]
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
    /// compare rhs
    EQT,
    /// "="
    /// equate to rhs
    EQ,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Argument {
    pub identifier: String,
    pub a_type: Types,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Literal {
    pub literal: String,
    pub l_type: Types,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Statement {
    pub action: String,
    pub parameters: Vec<Node>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
    pub identifier: Vec<String>,
    pub return_type: Types,
    pub arguments: Vec<Argument>,
    pub nodes: Vec<Node>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionCall {
    pub identifier: Vec<String>,
    pub arguments: Vec<Vec<Node>>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableAssignment {
    pub identifier: Vec<String>,
    pub value: Box<Vec<Node>>,
    pub immutability: bool,
    pub publicity: bool,
    pub type_data: Types,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expression {
    pub expr: Vec<String>, // maybe node? idk
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConditionBlock {
    pub keyword: String,
    pub parameters: Vec<Node>,
    pub nodes: Vec<Node>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MatchCase {
    pub case: Vec<Node>,
    pub block: Vec<Node>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Match {
    pub condition: Vec<Node>,
    pub block: Vec<MatchCase>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructMember {
    pub identifier: Vec<String>,
    pub t_mem: Types,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Struct {
    pub identifier: Vec<String>,
    pub members: Vec<StructMember>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructValue {
    pub identifier: Vec<String>,
    pub values: Vec<Node>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Enum {
    pub identifier: Vec<String>,
    pub members: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Math {
    pub lhs: Vec<Node>,
    pub rhs: Vec<Node>,
    pub operation: Operations,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExternC {
    pub block: String,
}

// todo ->
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NodeData {
    Statement(Statement),
    Function(Function),
    FunctionCall(FunctionCall),
    VariableAssignment(VariableAssignment),
    Expression(Expression),
    ConditionBlock(ConditionBlock),
    Match(Match),
    Literal(Literal),
    Math(Math),
    Struct(Struct),
    Enum(Enum),
    ExternC(ExternC),
    StructValue(StructValue),
    None,
}

// todo: if by the end of the parser, all node has is "data", just make Node NodeData
#[derive(PartialEq, Eq, Clone)]
pub struct Node {
    pub data: NodeData,
    pub line: i32,
    pub column: i32,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.data))
    }
}

impl Node {
    pub fn new(data: NodeData, line: i32, column: i32) -> Self {
        Self { data, line, column }
    }
    fn blank() -> Self {
        Self {
            data: NodeData::None,
            line: 0,
            column: 0,
        }
    }
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
