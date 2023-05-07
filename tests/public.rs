use comrade::{lexer::Lexer, node, Literal, Node, Types, VariableAssignment};

#[test]
fn test() {
    let lexer = Lexer::new("public const a = 5".to_string());
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![node!(
            variable_assignment,
            VariableAssignment {
                identifier: vec!["a".to_string()],
                immutability: true,
                publicity: true,
                type_data: String::new(),
                value: Box::new(vec![node!(
                    literal,
                    Literal {
                        literal: "5".to_string(),
                        l_type: Types::I32
                    }
                )])
            }
        )]
    );
    let lexer = Lexer::new("public let a = 5".to_string());
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![node!(
            variable_assignment,
            VariableAssignment {
                identifier: vec!["a".to_string()],
                immutability: false,
                publicity: true,
                type_data: String::new(),
                value: Box::new(vec![node!(
                    literal,
                    Literal {
                        literal: "5".to_string(),
                        l_type: Types::I32
                    }
                )])
            }
        )]
    );
}
