use comrade::parser::{Parser, ParserData};
use comrade::{lexer::Lexer, run, Enum, Node, NodeData};

#[test]
fn test() {
    assert_eq!(
        run!(
            "
enum Emotions {
    Good,
    Neutral,
    Awful,
    WorstEver
}
        "
        ),
        vec![Node::new(
            NodeData::Enum(Enum {
                identifier: vec!["Emotions".to_string()],
                members: vec![
                    "Good".to_string(),
                    "Neutral".to_string(),
                    "Awful".to_string(),
                    "WorstEver".to_string()
                ]
            }),
            0,
            0
        )]
    )
}
