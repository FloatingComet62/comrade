use super::{get_till_token_or_block, has, Expression, Math, Mode, Node, Operations};

pub fn parser(
    program: &mut Vec<Node>,
    text: &String,
    data: (usize, Vec<String>, Vec<String>, bool),
    input: &Vec<String>,
    i: usize,
) -> usize {
    let mut operator = "";
    let mut operation = Operations::NULL;
    if has(&data.1, vec!["+"], Mode::OR) {
        operator = "+";
        operation = Operations::ADD;
    }
    if has(&data.1, vec!["-"], Mode::OR) {
        operator = "-";
        operation = Operations::SUB;
    }
    if has(&data.1, vec!["*"], Mode::OR) {
        operator = "*";
        operation = Operations::MUL;
    }
    if has(&data.1, vec!["/"], Mode::OR) {
        operator = "/";
        operation = Operations::DIV;
    }
    if has(&data.1, vec![">"], Mode::OR) {
        operator = ">";
        operation = Operations::GR;
    }
    if has(&data.1, vec!["<"], Mode::OR) {
        operator = "<";
        operation = Operations::LT;
    }
    if has(&data.1, vec![">="], Mode::OR) {
        operator = ">=";
        operation = Operations::EQGR;
    }
    if has(&data.1, vec!["<="], Mode::OR) {
        operator = "<=";
        operation = Operations::EQLT;
    }

    let mut lhs = vec![text.to_string()];

    let mut raw_lhs = get_till_token_or_block(operator, &input, i);
    let rhs = get_till_token_or_block("EOL", &input, raw_lhs.0);

    lhs.append(&mut raw_lhs.1);
    program.push(Node::new(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Math {
            lhs: Expression { expr: lhs },
            rhs: Expression { expr: rhs.1 },
            operation,
        }),
    ));
    rhs.0 // skip to next and ignore the data
}
