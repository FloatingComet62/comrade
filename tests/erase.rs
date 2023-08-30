use comrade::parser::{Parser, ParserData};
use comrade::run;
use comrade::{lexer::Lexer, Expression, Node, NodeData, Statement};

#[test]
fn test() {
    assert_eq!(
        run!(
            "
let a = 5
erase a
        "
        )[1],
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
