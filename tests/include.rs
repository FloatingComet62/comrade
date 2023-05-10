use comrade::{lexer::Lexer, str_list_to_string_list, Expression, Node, NodeData, Statement};

#[test]
fn test() {
    let lexer = Lexer::new(
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
            Node::new(
                NodeData::Statement(Statement {
                    action: "include".to_string(),
                    parameters: vec![Node::new(
                        NodeData::Expression(Expression {
                            expr: str_list_to_string_list(vec!["std", "io"])
                        }),
                        0,
                        0
                    ),]
                }),
                0,
                0
            ),
            Node::new(
                NodeData::Statement(Statement {
                    action: "include".to_string(),
                    parameters: vec![Node::new(
                        NodeData::Expression(Expression {
                            expr: str_list_to_string_list(vec!["std", "math"])
                        }),
                        0,
                        0
                    )]
                }),
                0,
                0
            )
        ]
    );
}
