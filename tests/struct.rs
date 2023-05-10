use comrade::{lexer::Lexer, Node, NodeData, Struct, StructMember, Types};

#[test]
fn test() {
    let lexer = Lexer::new(
        "
struct User {
    name -> str
    age -> i32
    email -> str
}
"
        .to_string(),
    );
    let program = lexer.parse(false, false, false, false);
    assert_eq!(
        program.0,
        vec![Node::new(
            NodeData::Struct(Struct {
                identifier: vec!["User".to_string()],
                members: vec![
                    StructMember {
                        identifier: vec!["name".to_string()],
                        t_mem: Types::Str
                    },
                    StructMember {
                        identifier: vec!["age".to_string()],
                        t_mem: Types::I32
                    },
                    StructMember {
                        identifier: vec!["email".to_string()],
                        t_mem: Types::Str
                    }
                ]
            }),
            0,
            0
        )]
    )
}
