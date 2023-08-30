use super::{
    parser::{Parser, ParserData},
    Node,
};

pub mod booleans;
pub mod enum_expr;
pub mod extern_c;
pub mod function;
pub mod function_call;
pub mod if_expr;
pub mod literal;
pub mod match_expr;
pub mod math;
pub mod statement;
pub mod struct_expr;
pub mod variable_assignment;
pub mod while_expr;

pub trait NodeInterferace<T> {
    fn check(&self, text: String) -> bool;
    fn parser(
        &self,
        parser: Parser,
        program: &mut Vec<Node>,
        data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
        text: &String,
        previous_text: &String,
        input: &Vec<String>,
        i: &mut usize,
        parser_data: &mut ParserData,
    );
    fn compiler(
        &self,
        data: T,
        semi_colon_needed: bool,
        is_inside_function_call: bool,
    ) -> Option<String>;
}
