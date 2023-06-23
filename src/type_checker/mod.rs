use crate::{errors::send_error, errors::Errors, Expression, Node, NodeData, Types};

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub t: Types,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Variable>,
    pub return_t: Types,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub variable: Option<Variable>,
    pub function: Option<Function>,
}

pub fn check_main(input: &Vec<Node>) {
    let mut identifiers = vec![];
    check(&mut identifiers, input);
}

macro_rules! iden {
    ($x: ident, $y: expr) => {{
        let mut ident = Identifier {
            variable: None,
            function: None,
        };
        ident.$x = Some($y);
        ident
    }};
}

fn find_in_identifier_data(name: &String, identifier_data: &Vec<Identifier>) -> Option<Identifier> {
    for iden in identifier_data.iter() {
        if let Some(fun) = &iden.function {
            if name == &fun.name {
                return Some(iden.clone());
            }
        }
        if let Some(var) = &iden.variable {
            if name == &var.name {
                return Some(iden.clone());
            }
        }
    }

    return None;
}

pub fn check(identifier_data: &mut Vec<Identifier>, input: &Vec<Node>) {
    for item in input.iter() {
        match &item.data {
            NodeData::Statement(_) => {}
            NodeData::Function(f) => identifier_data.push(iden!(
                function,
                Function {
                    name: f.identifier.join("___"),
                    arguments: f
                        .arguments
                        .iter()
                        .map(|x| {
                            Variable {
                                name: x.identifier.clone(),
                                t: x.a_type,
                            }
                        })
                        .collect(),
                    return_t: f.return_type
                }
            )),
            NodeData::FunctionCall(fc) => {
                let fun_definition =
                    find_in_identifier_data(&fc.identifier.join("___"), &identifier_data)
                        .unwrap_or(Identifier {
                            variable: None,
                            function: None,
                        })
                        .function
                        .unwrap_or_else(|| {
                            send_error(
                                Errors::UNDEFINEDFUNCTION,
                                format!("Unknown function {}", fc.identifier.join("->")),
                                item.line,
                                item.column,
                            )
                        });
                for (i, arg) in fc.arguments.iter().enumerate() {
                    let arg_type = get_self_type(
                        &Node::new(
                            NodeData::Statement(crate::Statement {
                                action: "return".to_string(),
                                parameters: arg.to_vec(),
                            }),
                            0,
                            0,
                        ),
                        identifier_data,
                    );
                    if arg_type.0 != fun_definition.arguments[i].t {
                        send_error(
                            Errors::INCORRECTTYPE,
                            format!(
                                "Incorrect type of argument {:?}, provided {:?}, expected {:?}",
                                fun_definition.arguments[i].name,
                                arg_type.0,
                                fun_definition.arguments[i].t
                            ),
                            item.line,
                            item.column,
                        )
                    }
                }
            }
            NodeData::VariableAssignment(va) => {
                let actual_type = match va.value.first() {
                    Some(n) => get_self_type(&n, &identifier_data),
                    None => send_error(
                        Errors::INCORRECTTYPE,
                        format!("Unknown type of variable {}", va.identifier.join("->")),
                        item.line,
                        item.column,
                    ),
                };
                if va.type_data != Types::None && va.type_data != actual_type.0 {
                    // explicit type != actual type
                    send_error(
                        Errors::INCORRECTTYPE,
                        format!(
                            "Not matching types, Explicit type: {:?}\nActual type: {:?}",
                            va.type_data, actual_type.0
                        ),
                        item.line,
                        item.column,
                    )
                }
                identifier_data.push(iden!(
                    variable,
                    Variable {
                        name: va.identifier.join("___"),
                        t: actual_type.0
                    }
                ));
            }
            NodeData::Expression(_) => todo!(),
            NodeData::ConditionBlock(_) => todo!(),
            NodeData::Match(_) => todo!(),
            NodeData::Literal(_) => todo!(),
            NodeData::Math(_) => todo!(),
            NodeData::Struct(_) => todo!(),
            NodeData::Enum(_) => todo!(),
            NodeData::ExternC(_) => todo!(),
            NodeData::StructValue(_) => todo!(),
            NodeData::None => todo!(),
        }
    }
}

/// Returns
/// * `type` - Type of node data
/// * `unpredicitable` - externC disallows comrade from insuring type safety
fn get_self_type(node: &Node, identifier_data: &Vec<Identifier>) -> (Types, bool) {
    match &node.data {
        NodeData::Statement(s) => {
            if s.action == "return" {
                // todo: idk maybe more than the first parameter?
                return get_self_type(
                    &s.parameters.first().unwrap_or_else(|| {
                        send_error(
                            Errors::INCORRECTTYPE,
                            "Missing Return statement".to_string(),
                            node.line,
                            node.column,
                        )
                    }),
                    identifier_data,
                );
            }
            (Types::None, false)
        }
        NodeData::Function(_) => (Types::None, false),
        NodeData::FunctionCall(fc) => {
            let iden = find_in_identifier_data(&fc.identifier.join("___"), identifier_data);
            (
                iden.unwrap_or(Identifier {
                    variable: None,
                    function: None,
                })
                .function
                .unwrap_or_else(|| {
                    send_error(
                        Errors::UNDEFINEDFUNCTION,
                        format!("Unknown function {}", fc.identifier.join("->")),
                        node.line,
                        node.column,
                    )
                })
                .return_t,
                false,
            )
        }
        NodeData::VariableAssignment(_) => (Types::None, false),
        NodeData::Expression(e) => {
            // expression is the container for identifiers
            let iden = find_in_identifier_data(&e.expr.join("___"), identifier_data)
                .unwrap_or_else(|| {
                    send_error(
                        Errors::UNDEFINEDIDENTIFIER,
                        format!("Unknown Identifier {}", e.expr.join("->")),
                        node.line,
                        node.column,
                    );
                })
                .variable
                .unwrap_or_else(|| {
                    send_error(
                        Errors::UNDEFINEDIDENTIFIER,
                        format!("Unknown Identifier {}", e.expr.join("->")),
                        node.line,
                        node.column,
                    );
                });
            (iden.t, false)
        }
        NodeData::ConditionBlock(_) => todo!(),
        NodeData::Match(_) => todo!(),
        NodeData::Literal(l) => (l.l_type, false),
        NodeData::Math(_) => todo!(),
        NodeData::Struct(_) => todo!(),
        NodeData::Enum(_) => todo!(),
        NodeData::ExternC(_) => (Types::None, true),
        NodeData::StructValue(_) => todo!(),
        NodeData::None => (Types::None, false),
    }
}
