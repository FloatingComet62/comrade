use comrade::{
    lexer::Parser, node, str_list_to_string_list, Expression, FunctionCall, Literal, Match,
    MatchCase, Node, Types, VariableAssignment,
};

#[test]
fn test() {
    let lexer = Parser::new(
        "
match io->in(i32) {
    5 => {
        let x = 3
        io->out(x)
    }
    69 => io->out(\"nice\")
    420 => io->out(\"nice\")
    default => io->out(\"bruh\")
}
"
        .to_string(),
    );
    let program = lexer.parse(false, false);
    assert_eq!(
        program,
        vec![node!(
            _match,
            Match {
                condition: vec![node!(
                    function_call,
                    FunctionCall {
                        identifier: str_list_to_string_list(vec!["io", "->", "in"]),
                        arguments: vec![vec![node!(
                            literal,
                            Literal {
                                literal: "i32".to_string(),
                                l_type: Types::Type
                            }
                        )]]
                    }
                )],
                block: vec![
                    MatchCase {
                        case: vec![node!(
                            literal,
                            Literal {
                                literal: "5".to_string(),
                                l_type: Types::I32
                            }
                        )],
                        block: vec![
                            node!(
                                variable_assignment,
                                VariableAssignment {
                                    identifier: vec!["x".to_string()],
                                    immutability: false,
                                    publicity: false,
                                    value: Box::new(vec![node!(
                                        literal,
                                        Literal {
                                            literal: "3".to_string(),
                                            l_type: Types::I32
                                        }
                                    )])
                                }
                            ),
                            node!(
                                function_call,
                                FunctionCall {
                                    identifier: str_list_to_string_list(vec!["io", "->", "out"]),
                                    arguments: vec![vec![node!(
                                        expression,
                                        Expression {
                                            expr: vec!["x".to_string()]
                                        }
                                    )]]
                                }
                            )
                        ]
                    },
                    MatchCase {
                        case: vec![node!(
                            literal,
                            Literal {
                                literal: "69".to_string(),
                                l_type: Types::I32
                            }
                        )],
                        block: vec![node!(
                            function_call,
                            FunctionCall {
                                identifier: str_list_to_string_list(vec!["io", "->", "out"]),
                                arguments: vec![vec![node!(
                                    literal,
                                    Literal {
                                        literal: "\"nice\"".to_string(),
                                        l_type: Types::Str
                                    }
                                )]]
                            }
                        )]
                    },
                    MatchCase {
                        case: vec![node!(
                            literal,
                            Literal {
                                literal: "420".to_string(),
                                l_type: Types::I32
                            }
                        )],
                        block: vec![node!(
                            function_call,
                            FunctionCall {
                                identifier: str_list_to_string_list(vec!["io", "->", "out"]),
                                arguments: vec![vec![node!(
                                    literal,
                                    Literal {
                                        literal: "\"nice\"".to_string(),
                                        l_type: Types::Str
                                    }
                                )]]
                            }
                        )]
                    },
                    MatchCase {
                        case: vec![node!(
                            expression,
                            Expression {
                                expr: vec!["default".to_string()]
                            }
                        )],
                        block: vec![node!(
                            function_call,
                            FunctionCall {
                                identifier: str_list_to_string_list(vec!["io", "->", "out"]),
                                arguments: vec![vec![node!(
                                    literal,
                                    Literal {
                                        literal: "\"bruh\"".to_string(),
                                        l_type: Types::Str
                                    }
                                )]]
                            }
                        )]
                    },
                ]
            }
        )]
    )
}
