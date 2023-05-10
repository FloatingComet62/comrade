use comrade::{lexer::Lexer, Enum, Node, NodeData};

#[test]
fn test() {
    let lexer = Lexer::new(
        "
enum Emotions {
    Good,
    Neutral,
    Awful,
    WorstEver
}
    "
        .to_string(),
    );
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
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
