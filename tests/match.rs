use comrade::parser::{Parser, ParserData};
use comrade::run;
use comrade::{
    lexer::Lexer, str_list_to_string_list, Expression, FunctionCall, Literal, Match, MatchCase,
    Node, NodeData, Types, VariableAssignment,
};

#[test]
fn test() {
    assert_eq!(
        run!(
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
        ),
        vec![Node::new(
            NodeData::Match(Match {
                condition: vec![Node::new(
                    NodeData::FunctionCall(FunctionCall {
                        identifier: str_list_to_string_list(vec!["io", "->", "in"]),
                        arguments: vec![vec![Node::new(
                            NodeData::Literal(Literal {
                                literal: "i32".to_string(),
                                l_type: Types::Type
                            }),
                            0,
                            0
                        )]]
                    }),
                    0,
                    0
                )],
                block: vec![
                    MatchCase {
                        case: vec![Node::new(
                            NodeData::Literal(Literal {
                                literal: "5".to_string(),
                                l_type: Types::I32
                            }),
                            0,
                            0
                        )],
                        block: vec![
                            Node::new(
                                NodeData::VariableAssignment(VariableAssignment {
                                    identifier: vec!["x".to_string()],
                                    immutability: false,
                                    publicity: false,
                                    type_data: Types::None,
                                    value: Box::new(vec![Node::new(
                                        NodeData::Literal(Literal {
                                            literal: "3".to_string(),
                                            l_type: Types::I32
                                        }),
                                        0,
                                        0
                                    )])
                                }),
                                0,
                                0
                            ),
                            Node::new(
                                NodeData::FunctionCall(FunctionCall {
                                    identifier: str_list_to_string_list(vec!["io", "->", "out"]),
                                    arguments: vec![vec![Node::new(
                                        NodeData::Expression(Expression {
                                            expr: vec!["x".to_string()]
                                        }),
                                        0,
                                        0
                                    )]]
                                }),
                                0,
                                0
                            )
                        ]
                    },
                    MatchCase {
                        case: vec![Node::new(
                            NodeData::Literal(Literal {
                                literal: "69".to_string(),
                                l_type: Types::I32
                            }),
                            0,
                            0
                        )],
                        block: vec![Node::new(
                            NodeData::FunctionCall(FunctionCall {
                                identifier: str_list_to_string_list(vec!["io", "->", "out"]),
                                arguments: vec![vec![Node::new(
                                    NodeData::Literal(Literal {
                                        literal: "\"nice\"".to_string(),
                                        l_type: Types::Str
                                    }),
                                    0,
                                    0
                                )]]
                            }),
                            0,
                            0
                        )]
                    },
                    MatchCase {
                        case: vec![Node::new(
                            NodeData::Literal(Literal {
                                literal: "420".to_string(),
                                l_type: Types::I32
                            }),
                            0,
                            0
                        )],
                        block: vec![Node::new(
                            NodeData::FunctionCall(FunctionCall {
                                identifier: str_list_to_string_list(vec!["io", "->", "out"]),
                                arguments: vec![vec![Node::new(
                                    NodeData::Literal(Literal {
                                        literal: "\"nice\"".to_string(),
                                        l_type: Types::Str
                                    }),
                                    0,
                                    0
                                )]]
                            }),
                            0,
                            0
                        )]
                    },
                    MatchCase {
                        case: vec![Node::new(
                            NodeData::Expression(Expression {
                                expr: vec!["default".to_string()]
                            }),
                            0,
                            0
                        )],
                        block: vec![Node::new(
                            NodeData::FunctionCall(FunctionCall {
                                identifier: str_list_to_string_list(vec!["io", "->", "out"]),
                                arguments: vec![vec![Node::new(
                                    NodeData::Literal(Literal {
                                        literal: "\"bruh\"".to_string(),
                                        l_type: Types::Str
                                    }),
                                    0,
                                    0
                                )]]
                            }),
                            0,
                            0
                        )]
                    },
                ]
            }),
            0,
            0
        )]
    )
}
