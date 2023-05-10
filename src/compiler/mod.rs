use crate::{Node, NodeData, Types};

mod _enum;
mod _match;
mod _struct;
mod condition_block;
mod expression;
mod function;
mod function_call;
mod math;
mod statement;
mod variable_assignment;

pub fn compiler(
    program: &Vec<Node>,
    init_code: String,
    semi_colon_needed: bool,
    is_inside_function_call: bool,
) -> String {
    let mut output = init_code;
    for i in 0..program.len() {
        let item = &mut program.clone()[i];
        // if is_inside_function_call {

        // passing lists inside function calls
        // issue:
        // int x[] = {1, 2, 3, 4, 5};
        // printf("%d\n", sum(x));
        //
        // // both give error
        // // printf("%d\n", sum({1, 2, 3, 4, 5}));
        // // int x[] = (int[]){1, 2, 3, 4, 5};
        //
        // printf("%d\n", sum((int[]){1, 2, 3, 4, 5}));
        // return 0;

        // } else {
        // }
        match &item.data {
            NodeData::Literal(l) => {
                if l.literal.contains('_') {
                    let enum_vals: Vec<&str> = l.literal.split('_').collect();
                    output += enum_vals[1];
                } else {
                    output += &l.literal;
                }
            }
            NodeData::ExternC(e) => {
                output += &e.block;
            }
            NodeData::VariableAssignment(va) => {
                output += &variable_assignment::compile(va);
            }
            NodeData::Function(f) => {
                output += &function::compile(f);
            }
            NodeData::Statement(s) => {
                output += &statement::compile(program, s);
            }
            NodeData::FunctionCall(fc) => {
                output += &function_call::compile(fc, semi_colon_needed);
            }
            NodeData::ConditionBlock(cb) => {
                output += &condition_block::compile(cb);
            }
            NodeData::Math(m) => {
                output += &math::compile(m, !is_inside_function_call);
            }
            NodeData::Match(mt) => {
                output += &_match::compile(mt, semi_colon_needed);
            }
            NodeData::Expression(ex) => {
                output += &expression::compile(ex);
            }
            NodeData::Enum(em) => {
                output += &_enum::compile(em);
            }
            NodeData::Struct(st) => {
                output += &_struct::compile(st);
            }
            _ => todo!(),
        }
    }

    output
}

pub fn type_to_c_type(t: &Types) -> (&str, bool) {
    match t {
        Types::Bool => ("bool", false),
        Types::I8 => ("int", false),
        Types::U8 => ("int", false),
        Types::I16 => ("int", false),
        Types::U16 => ("int", false),
        Types::I32 => ("int", false),
        Types::U32 => ("int", false),
        Types::I64 => ("int", false),
        Types::U64 => ("int", false),
        Types::U128 => ("int", false),
        Types::I128 => ("int", false),
        Types::I8List => ("int", true),
        Types::U8List => ("int", true),
        Types::I16List => ("int", true),
        Types::U16List => ("int", true),
        Types::I32List => ("int", true),
        Types::U32List => ("int", true),
        Types::I64List => ("int", true),
        Types::U64List => ("int", true),
        Types::U128List => ("int", true),
        Types::I128List => ("int", true),
        Types::Str => ("char*", false),
        Types::StrList => ("char*", true),
        _ => ("void", false),
    }
}
