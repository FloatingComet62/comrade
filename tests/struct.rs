use comrade::parser::{Parser, ParserData};
use comrade::{lexer::Lexer, run, Node, NodeData, Struct, StructMember, Types};

#[test]
fn test() {
    assert_eq!(
        run!(
            "
struct User {
    name -> str
    age -> i32
    email -> str
}
        "
        ),
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
