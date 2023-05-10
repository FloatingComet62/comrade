use super::{get_till_token_or_block_and_math_block, load, FunctionCall, Node, NodeData};

pub fn parser(
    program: &mut Vec<Node>,
    text: &String,
    input: &Vec<String>,
    i: usize,
    identifiers: &mut Vec<Vec<String>>,
    enum_values: &mut Vec<Vec<String>>,
    struct_data: &mut Vec<Vec<String>>,
) -> usize {
    let mut identifier = vec![text.to_string()];
    let mut raw_identifier = get_till_token_or_block_and_math_block("(", input, i, false);

    identifier.append(&mut raw_identifier.1);

    let mut raw_args = vec![];
    let mut raw_raw_args =
        get_till_token_or_block_and_math_block(")", input, raw_identifier.0 - 1, false);

    // basically, join the block you found with the main content
    raw_args.append(&mut raw_raw_args.1);
    raw_args.append(&mut raw_raw_args.2);

    // Raw_args: ["(", "5"]
    // if there is only 1 "(", then it's fine
    // but if there is more, than the missing ")" at the end messes up the program
    // eg. ["(", "fib", "(", "5" , ")"]
    let mut brack_count = 0;
    for cell in raw_args.iter() {
        if cell == "(" {
            brack_count += 1;
        }
    }

    if brack_count > 1 {
        raw_args.push(")".to_string());
    }

    let mut args: Vec<Vec<Node>> = vec![];
    let mut arg: Vec<String> = vec![];
    for item in &raw_args {
        if item == "," {
            if !arg.is_empty() {
                arg.push("EOL".to_string());
                args.push(load(&arg, identifiers, enum_values, struct_data));
            }
            arg = vec![];
            continue;
        }
        arg.push(item.to_string());
    }
    if !arg.is_empty() {
        arg.push("EOL".to_string());
        args.push(load(&arg, identifiers, enum_values, struct_data));
    }
    program.push(Node::new(
        NodeData::FunctionCall(FunctionCall {
            identifier,
            arguments: args,
        }),
        0,
        0,
    ));
    raw_raw_args.0
}
