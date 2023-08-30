use comrade::parser::{Parser, ParserData};
use comrade::{lexer::Lexer, run, Literal, Node, NodeData, Types, VariableAssignment};

#[test]
fn test() {
    assert_eq!(
        run!("public const a = 5"),
        vec![Node::new(
            NodeData::VariableAssignment(VariableAssignment {
                identifier: vec!["a".to_string()],
                immutability: true,
                publicity: true,
                type_data: Types::None,
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
        )]
    );
    assert_eq!(
        run!("public let a = 5"),
        vec![Node::new(
            NodeData::VariableAssignment(VariableAssignment {
                identifier: vec!["a".to_string()],
                immutability: false,
                publicity: true,
                type_data: Types::None,
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
        )]
    );
}
