use comrade::{lexer::Lexer, Expression, Node, NodeData, Statement};

#[test]
fn test() {
    let lexer = Lexer::new(
        "
let a = 5
erase a
"
        .to_string(),
    );
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0[1],
        Node::new(
            NodeData::Statement(Statement {
                action: "erase".to_string(),
                parameters: vec![Node::new(
                    NodeData::Expression(Expression {
                        expr: vec!["a".to_string()]
                    }),
                    0,
                    0
                )]
            }),
            0,
            0
        )
    );
}
