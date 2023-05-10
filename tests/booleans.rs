use comrade::{lexer::Lexer, Literal, Node, NodeData, Types};

#[test]
fn test() {
    let lexer = Lexer::new("true".to_string());
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![Node::new(
            NodeData::Literal(Literal {
                literal: "true".to_string(),
                l_type: Types::Bool
            }),
            0,
            0
        )]
    );
    let lexer = Lexer::new("false".to_string());
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![Node::new(
            NodeData::Literal(Literal {
                literal: "false".to_string(),
                l_type: Types::Bool
            }),
            0,
            0
        )]
    );
}
