use crate::{Node, Types};

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
    program: &mut Vec<Node>,
    init_code: String,
    is_inside_function_call: bool,
) -> String {
    let mut output = init_code.clone();
    for i in 0..program.len() {
        let item = &mut program.clone()[i];
        // macro_rules! node_type_check {
        //     ($x: ident, $data: ident) => {
        //         if let Some(x) = &item.$x {
        //             output += &$data;
        //         }
        //     };
        // }
        if is_inside_function_call {
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
        } else {
        }
        if let Some(l) = &item.literal {
            if l.literal.contains("_") {
                let enum_vals: Vec<&str> = l.literal.split("_").collect();
                output += &enum_vals[1];
            } else {
                output += &l.literal;
            }
        }
        if let Some(x) = &item.extern_c {
            output += &x.block;
        }
        if let Some(x) = &item.variable_assignment {
            output += &variable_assignment::compile(x);
        }
        if let Some(x) = &mut item.function {
            output += &function::compile(x);
        }
        if let Some(x) = &mut item.statement {
            output += &statement::compile(program, x);
        }
        if let Some(x) = &item.function_call {
            output += &function_call::compile(x);
        }
        if let Some(x) = &mut item.condition_block {
            output += &condition_block::compile(x);
        }
        if let Some(x) = &mut item.math {
            output += &math::compile(x);
        }
        if let Some(x) = &mut item._match {
            output += &_match::compile(x);
        }
        if let Some(x) = &item.expression {
            output += &expression::compile(x);
        }
        if let Some(x) = &item._enum {
            output += &_enum::compile(x);
        }
        if let Some(x) = &item._struct {
            output += &_struct::compile(x);
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
