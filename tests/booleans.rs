use comrade::parser::{Parser, ParserData};
use comrade::{lexer::Lexer, run, Literal, Node, NodeData, Types};

#[test]
fn test() {
    assert_eq!(
        run!("true"),
        vec![Node::new(
            NodeData::Literal(Literal {
                literal: "true".to_string(),
                l_type: Types::Bool
            }),
            0,
            0
        )]
    );
    assert_eq!(
        run!("false"),
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
