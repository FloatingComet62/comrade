use comrade::{lexer::Parser, node, Literal, Node, Types};

#[test]
fn test() {
    let lexer = Parser::new("true".to_string());
    let program = lexer.parse(false, false);
    assert_eq!(
        program,
        vec![node!(
            literal,
            Literal {
                literal: "true".to_string(),
                l_type: Types::Bool
            }
        )]
    );
    let lexer = Parser::new("false".to_string());
    let program = lexer.parse(false, false);
    assert_eq!(
        program,
        vec![node!(
            literal,
            Literal {
                literal: "false".to_string(),
                l_type: Types::Bool
            }
        )]
    );
}
