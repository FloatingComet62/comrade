use comrade::{
    lexer::Parser, node, Expression, FunctionCall, Literal, Node, Statement, Types,
    VariableAssignment,
};

#[test]
fn test() {
    let lexer = Parser::new(
        "
let a = 5
someRandomFunction(a, \"Hello World\", 35, {
    let x = 5
    return x
})
    "
        .to_string(),
    );
    let program = lexer.parse(false, false, false);
    assert_eq!(
        program[1], // not checking the let statement
        node!(
            function_call,
            FunctionCall {
                identifier: vec!["someRandomFunction".to_string()],
                arguments: vec![
                    vec![node!(
                        expression,
                        Expression {
                            expr: vec!["a".to_string()]
                        }
                    )],
                    vec![node!(
                        literal,
                        Literal {
                            literal: "\"Hello World\"".to_string(),
                            l_type: Types::Str
                        }
                    )],
                    vec![node!(
                        literal,
                        Literal {
                            literal: "35".to_string(),
                            l_type: Types::I32
                        }
                    )],
                    vec![
                        node!(
                            variable_assignment,
                            VariableAssignment {
                                identifier: vec!["x".to_string()],
                                immutability: false,
                                publicity: false,
                                value: Box::new(vec![node!(
                                    literal,
                                    Literal {
                                        literal: "5".to_string(),
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
                                        expr: vec!["x".to_string()]
                                    }
                                )]
                            }
                        )
                    ]
                ]
            }
        )
    )
}
