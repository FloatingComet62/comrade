use comrade::{
    lexer::Lexer, ConditionBlock, FunctionCall, Literal, Math, Node, NodeData, Operations, Types,
};

#[test]
fn test() {
    let lexer = Lexer::new(
        "
if 5 > 5 {
    io->out(\"Hello World\")
}
"
        .to_string(),
    );
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![Node::new(
            NodeData::ConditionBlock(ConditionBlock {
                keyword: "if".to_string(),
                parameters: vec![Node::new(
                    NodeData::Math(Math {
                        lhs: vec![Node::new(
                            NodeData::Literal(Literal {
                                literal: "5".to_string(),
                                l_type: Types::I32
                            }),
                            0,
                            0
                        )],
                        rhs: vec![Node::new(
                            NodeData::Literal(Literal {
                                literal: "5".to_string(),
                                l_type: Types::I32
                            }),
                            0,
                            0
                        )],
                        operation: Operations::GR
                    }),
                    0,
                    0
                )],
                nodes: vec![Node::new(
                    NodeData::FunctionCall(FunctionCall {
                        identifier: vec!["io".to_string(), "->".to_string(), "out".to_string()],
                        arguments: vec![vec![Node::new(
                            NodeData::Literal(Literal {
                                literal: "\"Hello World\"".to_string(),
                                l_type: Types::Str
                            }),
                            0,
                            0
                        )]]
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
