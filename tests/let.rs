use comrade::{lexer::Lexer, Literal, Node, NodeData, Types, VariableAssignment};

#[test]
fn test() {
    let lexer = Lexer::new("let a = 5".to_string());
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![Node::new(
            NodeData::VariableAssignment(VariableAssignment {
                identifier: vec!["a".to_string()],
                immutability: false,
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

    let lexer = Lexer::new("let a -> u32 = 5".to_string());
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![Node::new(
            NodeData::VariableAssignment(VariableAssignment {
                identifier: vec!["a".to_string()],
                immutability: false,
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
    )
}
