use comrade::{
    lexer::Parser, node, ConditionBlock, FunctionCall, Literal, Math, Node, Operations, Types,
};

#[test]
fn test() {
    let lexer = Parser::new(
        "
if 5 > 5 {
    io->out(\"Hello World\")
}
"
        .to_string(),
    );
    let program = lexer.parse(false, false, false);
    assert_eq!(
        program,
        vec![node!(
            condition_block,
            ConditionBlock {
                keyword: "if".to_string(),
                parameters: vec![node!(
                    math,
                    Math {
                        lhs: vec![node!(
                            literal,
                            Literal {
                                literal: "5".to_string(),
                                l_type: Types::I32
                            }
                        )],
                        rhs: vec![node!(
                            literal,
                            Literal {
                                literal: "5".to_string(),
                                l_type: Types::I32
                            }
                        )],
                        operation: Operations::GR
                    }
                )],
                nodes: vec![node!(
                    function_call,
                    FunctionCall {
                        identifier: vec!["io".to_string(), "->".to_string(), "out".to_string()],
                        arguments: vec![vec![node!(
                            literal,
                            Literal {
                                literal: "\"Hello World\"".to_string(),
                                l_type: Types::Str
                            }
                        )]]
                    }
                )]
            }
        )]
    );
}
