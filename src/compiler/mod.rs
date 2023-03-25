use crate::{Node, Types};

mod function;
mod function_call;
mod statement;
mod variable_assignment;

pub fn compiler(program: &mut Vec<Node>, is_inside_function_call: bool) -> String {
    let mut output = String::new();
    for i in 0..program.len() {
        let item = &mut program.clone()[i];
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
            //printf("%d\n", sum((int[]){1, 2, 3, 4, 5}));
            //return 0;
            if let Some(l) = &item.literal {
                output += &l.literal;
            }
        } else {
            if let Some(l) = &item.literal {
                output += &l.literal;
            }
        }
        if let Some(e_c) = &item.extern_c {
            output += &e_c.block;
        }
        if let Some(va) = &item.variable_assignment {
            output += &variable_assignment::compile(va);
        }
        if let Some(fun) = &mut item.function {
            output += &function::compile(fun);
        }
        if let Some(s) = &mut item.statement {
            output += &statement::compile(program, s);
        }
        if let Some(fc) = &item.function_call {
            output += &function_call::compile(fc);
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
