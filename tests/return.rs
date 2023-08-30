use comrade::parser::{Parser, ParserData};
use comrade::{
    lexer::Lexer, run, str_list_to_string_list, Expression, Literal, Node, NodeData, Statement,
    Types, VariableAssignment,
};

#[test]
fn test() {
    assert_eq!(
        run!("return 2"),
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

    assert_eq!(
        run!(
            "
let a = 2
return a
        "
        ),
        vec![
            Node::new(
                NodeData::VariableAssignment(VariableAssignment {
                    identifier: str_list_to_string_list(vec!["a"]),
                    immutability: false,
                    publicity: false,
                    type_data: Types::None,
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
