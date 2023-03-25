use comrade::{lexer::Parser, node, str_list_to_string_list, Expression, Node, Statement};

#[test]
fn test() {
    let lexer = Parser::new(
        "
include std->io
include std->math
"
        .to_string(),
    );
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![
            node!(
                statement,
                Statement {
                    action: "include".to_string(),
                    parameters: vec![node!(
                        expression,
                        Expression {
                            expr: str_list_to_string_list(vec!["std", "io"])
                        }
                    ),]
                }
            ),
            node!(
                statement,
                Statement {
                    action: "include".to_string(),
                    parameters: vec![node!(
                        expression,
                        Expression {
                            expr: str_list_to_string_list(vec!["std", "math"])
                        }
                    )]
                }
            )
        ]
    );
}
