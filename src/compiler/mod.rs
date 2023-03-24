use crate::Node;

mod variable_assignment;

pub fn compiler(program: &Vec<Node>) -> String {
    let mut output = String::new();
    for item in program {
        if let Some(e_c) = &item.extern_c {
            output += &e_c.block;
        }
        if let Some(va) = &item.variable_assignment {
            output += &variable_assignment::parser(va);
        }
    }

    output
}
