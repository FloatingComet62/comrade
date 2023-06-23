use comrade::{
    lexer::Lexer, str_list_to_string_list, ConditionBlock, Expression, FunctionCall, Literal, Math,
    Node, NodeData, Operations, Types, VariableAssignment,
};

#[test]
fn test() {
    let lexer = Lexer::new(
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
            Node::new(
                NodeData::VariableAssignment(VariableAssignment {
                    identifier: vec!["i".to_string()],
                    immutability: false,
                    publicity: false,
                    type_data: Types::None,
                    value: Box::new(vec![Node::new(
                        NodeData::Literal(Literal {
                            literal: "0".to_string(),
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
                NodeData::ConditionBlock(ConditionBlock {
                    keyword: "while".to_string(),
                    parameters: vec![Node::new(
                        NodeData::Math(Math {
                            lhs: vec![Node::new(
                                NodeData::Expression(Expression {
                                    expr: vec!["i".to_string()]
                                }),
                                0,
                                0
                            )],
                            rhs: vec![Node::new(
                                NodeData::Literal(Literal {
                                    literal: "5".to_string(),
                                    l_type: Types::I32
                                }),
                                0,
                                0
                            )],
                            operation: Operations::NEQ
                        }),
                        0,
                        0
                    )],
                    nodes: vec![
                        Node::new(
                            NodeData::FunctionCall(FunctionCall {
                                identifier: str_list_to_string_list(vec!["io", "->", "out"]),
                                arguments: vec![vec![Node::new(
                                    NodeData::Expression(Expression {
                                        expr: vec!["i".to_string()]
                                    }),
                                    0,
                                    0
                                )]],
                            }),
                            0,
                            0
                        ),
                        Node::new(
                            NodeData::Math(Math {
                                lhs: vec![Node::new(
                                    NodeData::Expression(Expression {
                                        expr: vec!["i".to_string()],
                                    }),
                                    0,
                                    0
                                )],
                                rhs: vec![Node::new(
                                    NodeData::Literal(Literal {
                                        literal: "1".to_string(),
                                        l_type: Types::I32
                                    }),
                                    0,
                                    0
                                )],
                                operation: Operations::ADDEQT
                            }),
                            0,
                            0
                        )
                    ]
                }),
                0,
                0
            ),
        ]
    )
}
