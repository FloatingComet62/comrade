use crate::node;

use super::{get_till_token_or_block, load, Match, MatchCase, Node};

pub fn get_match_case(
    i: usize,
    input: &Vec<String>,
    mut identifiers: &mut Vec<Vec<String>>,
    mut enum_values: &mut Vec<Vec<String>>,
    mut struct_data: &mut Vec<Vec<String>>,
) -> (usize, Option<MatchCase>) {
    let case_data = get_till_token_or_block("=>", &input, i, false);
    let expr_data = get_till_token_or_block("EOL", &input, case_data.0, false);
    let case_target;
    if case_data.3 {
        if case_data.2.len() == 0 {
            return (case_data.0, None);
        }
        case_target = case_data.2;
    } else {
        if case_data.1.len() == 0 {
            return (case_data.0, None);
        }
        case_target = case_data.1
    }
    let expr_target;
    if expr_data.3 {
        if expr_data.2.len() == 0 {
            return (case_data.0, None);
        }
        expr_target = expr_data.2;
    } else {
        if expr_data.1.len() == 0 {
            return (case_data.0, None);
        }
        expr_target = expr_data.1
    }
    return (
        expr_data.0,
        Some(MatchCase {
            block: load(
                &expr_target,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
            ),
            case: load(
                &case_target,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data,
            ),
        }),
    );
}

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    mut identifiers: &mut Vec<Vec<String>>,
    mut enum_values: &mut Vec<Vec<String>>,
    mut struct_data: &mut Vec<Vec<String>>,
) -> usize {
    let mut block = vec![];
    let mut j = 0;
    while j < data.2.len() {
        let x = get_match_case(
            j,
            &data.2,
            &mut identifiers,
            &mut enum_values,
            &mut struct_data,
        );
        if let Some(y) = x.1 {
            block.push(y);
        }
        j = x.0;
    }

    program.push(node!(
        _match,
        Match {
            condition: load(
                &data.1,
                &mut identifiers,
                &mut enum_values,
                &mut struct_data
            ),
            block,
        }
    ));
    data.0 // skip to next and ignore the data
}
