use comrade::{lexer::Parser, node, Expression, Node, Statement};

#[test]
fn test() {
    let lexer = Parser::new(
        "
let a = 5
erase a
"
        .to_string(),
    );
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0[1],
        node!(
            statement,
            Statement {
                action: "erase".to_string(),
                parameters: vec![node!(
                    expression,
                    Expression {
                        expr: vec!["a".to_string()]
                    }
                )]
            }
        )
    );
}
