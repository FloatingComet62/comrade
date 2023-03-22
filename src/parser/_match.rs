use super::{get_till_token_or_block, load, Match, MatchCase, Node};

pub fn get_match_case(i: usize, input: &Vec<String>) -> (usize, Option<MatchCase>) {
    let case_data = get_till_token_or_block("=>", &input, i);
    if case_data.3 {
        if case_data.2.len() == 0 {
            return (case_data.0, None);
        }
        return (
            case_data.0,
            Some(MatchCase {
                block: load(&case_data.2),
                case: case_data.1,
            }),
        );
    } else {
        let expr_data = get_till_token_or_block("EOL", &input, case_data.0);
        if case_data.1.len() == 0 {
            return (case_data.0, None);
        }
        return (
            expr_data.0,
            Some(MatchCase {
                block: load(&expr_data.1),
                case: case_data.1,
            }),
        );
    }
}

pub fn parser(program: &mut Vec<Node>, data: (usize, Vec<String>, Vec<String>, bool)) -> usize {
    let mut block = vec![];
    let mut j = 0;
    while j < data.2.len() {
        let x = get_match_case(j, &data.2);
        if let Some(y) = x.1 {
            block.push(y);
        }
        j = x.0;
    }

    program.push(Node::new(
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Match {
            condition: load(&data.1),
            block,
        }),
        None,
        None,
        None,
        None,
    ));
    data.0 // skip to next and ignore the data
}
