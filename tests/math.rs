use comrade::parser::{Parser, ParserData};
use comrade::run;
use comrade::{lexer::Lexer, Literal, Math, Node, NodeData, Operations, Types};

macro_rules! math_test {
    ($op_str: expr, $op: expr) => {
        assert_eq!(
            run!(format!("5 {} 5", $op_str)),
            vec![Node::new(
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
                    operation: $op
                }),
                0,
                0
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
