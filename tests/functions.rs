use comrade::{
    lexer::Parser, node, Argument, Expression, Function, Literal, Math, Node, Operations,
    Statement, Types,
};

#[test]
fn test() {
    let lexer = Parser::new(
        "
fun add_3(x -> u32) => u32 {
    return x + 3
}
"
        .to_string(),
    );
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![node!(
            function,
            Function {
                identifier: vec!["add_3".to_string()],
                arguments: vec![Argument {
                    identifier: "x".to_string(),
                    a_type: Types::U32
                }],
                return_type: Types::U32,
                nodes: vec![node!(
                    statement,
                    Statement {
                        action: "return".to_string(),
                        parameters: vec![node!(
                            math,
                            Math {
                                lhs: vec![node!(
                                    expression,
                                    Expression {
                                        expr: vec!["x".to_string()]
                                    }
                                )],
                                rhs: vec![node!(
                                    literal,
                                    Literal {
                                        literal: "3".to_string(),
                                        l_type: Types::I32
                                    }
                                )],
                                operation: Operations::ADD
                            }
                        )]
                    }
                )]
            }
        )]
    );
}
