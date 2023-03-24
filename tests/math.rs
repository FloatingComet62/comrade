use comrade::{lexer::Parser, node, Literal, Math, Node, Operations, Types};

#[test]
fn test() {
    let lexer = Parser::new("5 == 5".to_string());
    let program = lexer.parse(false, false, false);
    assert_eq!(
        program,
        vec![node!(
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
                operation: Operations::EQT
            }
        )]
    );

    let lexer = Parser::new("5 > 5".to_string());
    let program = lexer.parse(false, false, false);
    assert_eq!(
        program,
        vec![node!(
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
        )]
    );

    let lexer = Parser::new("5 < 5".to_string());
    let program = lexer.parse(false, false, false);
    assert_eq!(
        program,
        vec![node!(
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
                operation: Operations::LT
            }
        )]
    );

    let lexer = Parser::new("5 >= 5".to_string());
    let program = lexer.parse(false, false, false);
    assert_eq!(
        program,
        vec![node!(
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
                operation: Operations::EQGR
            }
        )]
    );

    let lexer = Parser::new("5 <= 5".to_string());
    let program = lexer.parse(false, false, false);
    assert_eq!(
        program,
        vec![node!(
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
                operation: Operations::EQLT
            }
        )]
    );

    let lexer = Parser::new("5 != 5".to_string());
    let program = lexer.parse(false, false, false);
    assert_eq!(
        program,
        vec![node!(
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
                operation: Operations::NEQ
            }
        )]
    );
}
