use super::{get_till_token_or_block_and_math_block, load, Match, MatchCase, Node, NodeData};

pub fn get_match_case(
    i: usize,
    input: &Vec<String>,
    identifiers: &mut Vec<Vec<String>>,
    enum_values: &mut Vec<Vec<String>>,
    struct_data: &mut Vec<Vec<String>>,
) -> (usize, Option<MatchCase>) {
    let case_data = get_till_token_or_block_and_math_block("=>", input, i, false);
    let expr_data = get_till_token_or_block_and_math_block("EOL", input, case_data.0, false);
    let case_target = if case_data.3 {
        if case_data.2.is_empty() {
            return (case_data.0, None);
        }
        case_data.2
    } else {
        if case_data.1.is_empty() {
            return (case_data.0, None);
        }
        case_data.1
    };
    let expr_target = if expr_data.3 {
        if expr_data.2.is_empty() {
            return (case_data.0, None);
        }
        expr_data.2
    } else {
        if expr_data.1.is_empty() {
            return (case_data.0, None);
        }
        expr_data.1
    };
    (
        expr_data.0,
        Some(MatchCase {
            block: load(&expr_target, identifiers, enum_values, struct_data),
            case: load(&case_target, identifiers, enum_values, struct_data),
        }),
    )
}

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool, Vec<String>),
    identifiers: &mut Vec<Vec<String>>,
    enum_values: &mut Vec<Vec<String>>,
    struct_data: &mut Vec<Vec<String>>,
) -> usize {
    let mut block = vec![];
    let mut j = 0;
    while j < data.2.len() {
        let x = get_match_case(j, &data.2, identifiers, enum_values, struct_data);
        if let Some(y) = x.1 {
            block.push(y);
        }
        j = x.0;
    }

    program.push(Node::new(
        NodeData::Match(Match {
            condition: load(&data.1, identifiers, enum_values, struct_data),
            block,
        }),
        0,
        0,
    ));
    data.0 // skip to next and ignore the data
}
