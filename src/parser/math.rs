use crate::node;

use super::{super::Operations, get_till_token_or_block, has, load, Math, Mode, Node};

pub fn parser(
    program: &mut Vec<Node>,
    text: &String,
    data: (usize, Vec<String>, Vec<String>, bool),
    input: &Vec<String>,
    i: usize,
    mut identifiers: &mut Vec<Vec<String>>,
    mut first_identifiers: &mut Vec<String>,
    mut enum_values: &mut Vec<Vec<String>>,
) -> usize {
    let mut operator = "";
    let mut operation = Operations::NULL;
    macro_rules! check {
        ($item: expr, $op: expr) => {
            if has(&data.1, vec![$item], Mode::OR) {
                operator = $item;
                operation = $op;
            }
        };
    }
    check!("+", Operations::ADD);
    check!("-", Operations::SUB);
    check!("*", Operations::MUL);
    check!("/", Operations::DIV);
    check!("==", Operations::EQT);
    check!(">=", Operations::EQGR);
    check!("<=", Operations::EQLT);
    check!(">", Operations::GR);
    check!("<", Operations::LT);
    check!("!=", Operations::NEQ);
    check!("+=", Operations::ADDEQT);
    check!("-=", Operations::SUBEQT);
    check!("*=", Operations::MULEQT);
    check!("/=", Operations::DIVEQT);

    let mut lhs = vec![text.to_string()];
    let mut raw_lhs = get_till_token_or_block(operator, &input, i, false);
    let rhs = get_till_token_or_block("EOL", &input, raw_lhs.0, false);
    lhs.append(&mut raw_lhs.1);
    program.push(node!(
        math,
        Math {
            lhs: load(
                &lhs,
                &mut identifiers,
                &mut first_identifiers,
                &mut enum_values
            ),
            rhs: load(
                &rhs.1,
                &mut identifiers,
                &mut first_identifiers,
                &mut enum_values
            ),
            operation,
        }
    ));
    rhs.0 // skip to next and ignore the data
}
