use super::{get_till_token_or_block, has, Expression, Math, Mode, Node, Operations};

pub fn parser(program: &mut Vec<Node>, text: &String, input: &Vec<String>, i: usize) -> usize {
    let mut operator = "+";
    let mut operation = Operations::ADD;
    if has(&input, vec!["+"], Mode::OR) {
        operator = "+";
        operation = Operations::ADD;
    }
    if has(&input, vec!["-"], Mode::OR) {
        operator = "-";
        operation = Operations::SUB;
    }
    if has(&input, vec!["*"], Mode::OR) {
        operator = "*";
        operation = Operations::MUL;
    }
    if has(&input, vec!["/"], Mode::OR) {
        operator = "/";
        operation = Operations::DIV;
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
