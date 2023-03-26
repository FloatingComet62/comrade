use comrade::{
    lexer::Parser, node, str_list_to_string_list, ConditionBlock, Expression, FunctionCall,
    Literal, Math, Node, Operations, Types, VariableAssignment,
};

#[test]
fn test() {
    let lexer = Parser::new(
        "
let i = 0
while i != 5 {
    io->out(i)
    i += 1
}
"
        .to_string(),
    );
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![
            node!(
                variable_assignment,
                VariableAssignment {
                    identifier: vec!["i".to_string()],
                    immutability: false,
                    publicity: false,
                    type_data: String::new(),
                    value: Box::new(vec![node!(
                        literal,
                        Literal {
                            literal: "0".to_string(),
                            l_type: Types::I32
                        }
                    )])
                }
            ),
            node!(
                condition_block,
                ConditionBlock {
                    keyword: "while".to_string(),
                    parameters: vec![node!(
                        math,
                        Math {
                            lhs: vec![node!(
                                expression,
                                Expression {
                                    expr: vec!["i".to_string()]
                                }
                            )],
                            rhs: vec![node!(
                                literal,
                                Literal {
                                    literal: "5".to_string(),
                                    l_type: Types::I32
                                }
                            )],
                            operation: Operations::NEQ
                        }
                    )],
                    nodes: vec![
                        node!(
                            function_call,
                            FunctionCall {
                                identifier: str_list_to_string_list(vec!["io", "->", "out"]),
                                arguments: vec![vec![node!(
                                    expression,
                                    Expression {
                                        expr: vec!["i".to_string()]
                                    }
                                )]],
                            }
                        ),
                        node!(
                            math,
                            Math {
                                lhs: vec![node!(
                                    expression,
                                    Expression {
                                        expr: vec!["i".to_string()],
                                    }
                                )],
                                rhs: vec![node!(
                                    literal,
                                    Literal {
                                        literal: "1".to_string(),
                                        l_type: Types::I32
                                    }
                                )],
                                operation: Operations::ADDEQT
                            }
                        )
                    ]
                }
            ),
        ]
    )
}
