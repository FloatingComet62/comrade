use comrade::{
    lexer::Lexer, Expression, FunctionCall, Literal, Node, NodeData, Statement, Types,
    VariableAssignment,
};

#[test]
fn test() {
    let lexer = Lexer::new(
        "
let a = 5
someRandomFunction(a, \"Hello World\", 35, {
    let x = 5
    return x
})
    "
        .to_string(),
    );
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0[1], // not checking the let statement
        Node::new(
            NodeData::FunctionCall(FunctionCall {
                identifier: vec!["someRandomFunction".to_string()],
                arguments: vec![
                    vec![Node::new(
                        NodeData::Expression(Expression {
                            expr: vec!["a".to_string()]
                        }),
                        0,
                        0
                    )],
                    vec![Node::new(
                        NodeData::Literal(Literal {
                            literal: "\"Hello World\"".to_string(),
                            l_type: Types::Str
                        }),
                        0,
                        0
                    )],
                    vec![Node::new(
                        NodeData::Literal(Literal {
                            literal: "35".to_string(),
                            l_type: Types::I32
                        }),
                        0,
                        0
                    )],
                    vec![
                        Node::new(
                            NodeData::VariableAssignment(VariableAssignment {
                                identifier: vec!["x".to_string()],
                                immutability: false,
                                publicity: false,
                                type_data: String::new(),
                                value: Box::new(vec![Node::new(
                                    NodeData::Literal(Literal {
                                        literal: "5".to_string(),
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
                            NodeData::Statement(Statement {
                                action: "return".to_string(),
                                parameters: vec![Node::new(
                                    NodeData::Expression(Expression {
                                        expr: vec!["x".to_string()]
                                    }),
                                    0,
                                    0
                                )]
                            }),
                            0,
                            0
                        )
                    ]
                ]
            }),
            0,
            0
        )
    )
}
