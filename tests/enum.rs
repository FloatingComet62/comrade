use comrade::{lexer::Parser, node, Enum, Node};

#[test]
fn test() {
    let lexer = Parser::new(
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
    let program = lexer.parse(false, false);
    assert_eq!(
        program,
        vec![node!(
            _enum,
            Enum {
                identifier: vec!["Emotions".to_string()],
                members: vec![
                    "Good".to_string(),
                    "Neutral".to_string(),
                    "Awful".to_string(),
                    "WorstEver".to_string()
                ]
            }
        )]
    )
}
