use comrade::parser::{Parser, ParserData};
use comrade::run;
use comrade::{
    lexer::Lexer, Argument, Expression, Function, Literal, Math, Node, NodeData, Operations,
    Statement, Types,
};

#[test]
fn test() {
    assert_eq!(
        run!(
            "
fun add_3(x -> u32) => u32 {
    return x + 3
}
        "
        ),
        vec![Node::new(
            NodeData::Function(Function {
                identifier: vec!["add_3".to_string()],
                arguments: vec![Argument {
                    identifier: "x".to_string(),
                    a_type: Types::U32
                }],
                return_type: Types::U32,
                nodes: vec![Node::new(
                    NodeData::Statement(Statement {
                        action: "return".to_string(),
                        parameters: vec![Node::new(
                            NodeData::Math(Math {
                                lhs: vec![Node::new(
                                    NodeData::Expression(Expression {
                                        expr: vec!["x".to_string()]
                                    }),
                                    0,
                                    0
                                )],
                                rhs: vec![Node::new(
                                    NodeData::Literal(Literal {
                                        literal: "3".to_string(),
                                        l_type: Types::I32
                                    }),
                                    0,
                                    0
                                )],
                                operation: Operations::ADD
                            }),
                            0,
                            0
                        )]
                    }),
                    0,
                    0
                )]
            }),
            0,
            0
        )]
    );
}
