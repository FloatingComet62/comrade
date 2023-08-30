use comrade::{
    lexer::Lexer,
    parser::{Parser, ParserData},
    run, Literal, Node, NodeData, Types, VariableAssignment,
};

#[test]
fn test() {
    assert_eq!(
        run!("const a = 5"),
        vec![Node::new(
            NodeData::VariableAssignment(VariableAssignment {
                identifier: vec!["a".to_string()],
                immutability: true,
                publicity: false,
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
        run!("const a -> u32 = 5"),
        vec![Node::new(
            NodeData::VariableAssignment(VariableAssignment {
                identifier: vec!["a".to_string()],
                immutability: true,
                publicity: false,
                type_data: Types::U32,
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
