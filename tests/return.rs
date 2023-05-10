use comrade::{
    lexer::Lexer, str_list_to_string_list, Expression, Literal, Node, NodeData, Statement, Types,
    VariableAssignment,
};

#[test]
fn test() {
    let lexer = Lexer::new("return 2".to_string());
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![Node::new(
            NodeData::Statement(Statement {
                action: "return".to_string(),
                parameters: vec![Node::new(
                    NodeData::Literal(Literal {
                        literal: "2".to_string(),
                        l_type: Types::I32
                    }),
                    0,
                    0
                )]
            }),
            0,
            0
        )]
    );

    let lexer = Lexer::new(
        "
let a = 2
return a
"
        .to_string(),
    );
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![
            Node::new(
                NodeData::VariableAssignment(VariableAssignment {
                    identifier: str_list_to_string_list(vec!["a"]),
                    immutability: false,
                    publicity: false,
                    type_data: String::new(),
                    value: Box::new(vec![Node::new(
                        NodeData::Literal(Literal {
                            literal: "2".to_string(),
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
                            expr: vec!["a".to_string()]
                        }),
                        0,
                        0
                    )]
                }),
                0,
                0
            )
        ]
    );
}
