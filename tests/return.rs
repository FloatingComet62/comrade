use comrade::{
    lexer::Parser, node, str_list_to_string_list, Expression, Literal, Node, Statement, Types,
    VariableAssignment,
};

#[test]
fn test() {
    let lexer = Parser::new("return 2".to_string());
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![node!(
            statement,
            Statement {
                action: "return".to_string(),
                parameters: vec![node!(
                    literal,
                    Literal {
                        literal: "2".to_string(),
                        l_type: Types::I32
                    }
                )]
            }
        )]
    );

    let lexer = Parser::new(
        "
let a = 2
return a
"
        .to_string(),
    );
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![
            node!(
                variable_assignment,
                VariableAssignment {
                    identifier: str_list_to_string_list(vec!["a"]),
                    immutability: false,
                    publicity: false,
                    type_data: String::new(),
                    value: Box::new(vec![node!(
                        literal,
                        Literal {
                            literal: "2".to_string(),
                            l_type: Types::I32
                        }
                    )])
                }
            ),
            node!(
                statement,
                Statement {
                    action: "return".to_string(),
                    parameters: vec![node!(
                        expression,
                        Expression {
                            expr: vec!["a".to_string()]
                        }
                    )]
                }
            )
        ]
    );
}
