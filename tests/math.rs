use comrade::{lexer::Lexer, node, Literal, Math, Node, Operations, Types};

macro_rules! math_test {
    ($op_str: expr, $op: expr) => {
        let lexer = Lexer::new(format!("5 {} 5", $op_str));
        let program = lexer.parse(false, false, false, false);
        assert_eq!(
            program.0,
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
                    operation: $op
                }
            )]
        );
    };
}

#[test]
fn test() {
    math_test!("+", Operations::ADD);
    math_test!("-", Operations::SUB);
    math_test!("*", Operations::MUL);
    math_test!("/", Operations::DIV);
    math_test!("+=", Operations::ADDEQT);
    math_test!("-=", Operations::SUBEQT);
    math_test!("*=", Operations::MULEQT);
    math_test!("/=", Operations::DIVEQT);
    math_test!(">", Operations::GR);
    math_test!("<", Operations::LT);
    math_test!(">=", Operations::EQGR);
    math_test!("<=", Operations::EQLT);
    math_test!("=", Operations::EQ);
    math_test!("==", Operations::EQT);
    math_test!("!=", Operations::NEQ);
}
