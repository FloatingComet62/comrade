use crate::{nodes::NodeInterferace, Node, NodeData, Types};

use super::nodes;

pub fn compiler(
    program: &Vec<Node>,
    init_code: String,
    semi_colon_needed: bool,
    is_inside_function_call: bool,
) -> String {
    let mut output = init_code;
    //let bm = nodes::booleans::BooleanManager::new();
    let em = nodes::enum_expr::EnumManager::new();
    let exm = nodes::extern_c::ExternCManager::new();
    let fcm = nodes::function_call::FunctionCallManager::new(vec![]);
    let fm = nodes::function::FunctionManager::new();
    let ifm = nodes::if_expr::IfElseManager::new();
    let lm = nodes::literal::LiteralManager::new(Types::None);
    let mm = nodes::match_expr::MatchManager::new();
    let mam = nodes::math::MathManager::new(vec![], vec![], false);
    let sm = nodes::statement::StatementManager::new();
    // let stm = nodes::statement::StatementManager::new();
    let strm = nodes::struct_expr::StructManager::new();
    let vam = nodes::variable_assignment::VariableAssignmentManager::new(false);
    // let wm = nodes::while_expr::WhileManager::new();

    for i in 0..program.len() {
        let item = &mut program.clone()[i];
        // if is_inside_function_call {

        // passing lists inside function calls
        // issue:
        // int x[] = {1, 2, 3, 4, 5};
        // printf("%d\n", sum(x));
        //
        // both give error
        // printf("%d\n", sum({1, 2, 3, 4, 5}));
        // int x[] = (int[]){1, 2, 3, 4, 5};
        //
        // printf("%d\n", sum((int[]){1, 2, 3, 4, 5}));
        // return 0;

        // } else {
        // }

        macro_rules! append {
            ($var: ident, $x: expr) => {
                output += &$var
                    .compiler($x, semi_colon_needed, is_inside_function_call)
                    .unwrap_or(String::new())
            };
        }

        match item.data.clone() {
            NodeData::Literal(x) => append!(lm, x),
            NodeData::ExternC(x) => append!(exm, x),
            NodeData::VariableAssignment(x) => append!(vam, x),
            NodeData::Function(x) => append!(fm, x),
            NodeData::Statement(x) => append!(sm, x),
            NodeData::FunctionCall(x) => append!(fcm, x),
            NodeData::ConditionBlock(x) => append!(ifm, x),
            NodeData::Math(x) => append!(mam, x),
            NodeData::Match(x) => append!(mm, x),
            NodeData::Expression(x) => {
                if x.expr == vec!["NULL".to_string()] {
                    return "NULL".to_string();
                }

                let mut to_append = String::new();
                let mut list_indexing = false;
                for (i, item) in x.expr.iter().enumerate() {
                    let blank = String::new();
                    let next = x.expr.get(i + 1).unwrap_or(&blank);
                    if next == "[" {
                        list_indexing = true;
                    }

                    to_append += item;
                    if !list_indexing && i != x.expr.len() - 1 {
                        to_append += ".";
                    }
                    if item == "]" {
                        //todo maybe next will also work?
                        list_indexing = false;
                    }
                }
                output += &to_append;
            }
            NodeData::Enum(x) => append!(em, x),
            NodeData::Struct(x) => append!(strm, x),
            _ => todo!(),
        }

        if is_inside_function_call && i != program.len() - 1 {
            output += ",";
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
