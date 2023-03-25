use super::{load, ConditionBlock, Node};
use crate::{exit, node};

pub fn parser(
    program: &mut Vec<Node>,
    data: (usize, Vec<String>, Vec<String>, bool),
    text: &String,
    previous_text: &String,
    input: &Vec<String>,
    i: usize,
    mut identifiers: &mut Vec<Vec<String>>,
    mut first_identifiers: &mut Vec<String>,
    mut enum_values: &mut Vec<Vec<String>>,
) -> usize {
    if text == "if" {
        if previous_text == "else" {
            let last_data = is_last_an_if(&program);
            if last_data == 0 {
                exit("Missing if part for else if", None);
            }
            program.push(node!(
                condition_block,
                ConditionBlock {
                    keyword: "else if".to_string(),
                    parameters: load(
                        &data.1,
                        &mut identifiers,
                        &mut first_identifiers,
                        &mut enum_values
                    ),
                    nodes: load(
                        &data.2,
                        &mut identifiers,
                        &mut first_identifiers,
                        &mut enum_values
                    ),
                }
            ));
            return data.0;
        }
        program.push(node!(
            condition_block,
            ConditionBlock {
                keyword: "if".to_string(),
                parameters: load(
                    &data.1,
                    &mut identifiers,
                    &mut first_identifiers,
                    &mut enum_values
                ),
                nodes: load(
                    &data.2,
                    &mut identifiers,
                    &mut first_identifiers,
                    &mut enum_values
                ),
            }
        ));
    }
    if text == "else" {
        // if next token is "if", then skip this, else if is handled above
        let blank = String::new();
        let next_token = input.get(i + 1).unwrap_or(&blank);
        if next_token == "if" {
            return i;
        }

        let last_node = is_last_an_if(&program);
        if last_node == 0 {
            exit("Missing if part for else", None);
        }
        program.push(node!(
            condition_block,
            ConditionBlock {
                keyword: "else".to_string(),
                parameters: vec![],
                nodes: load(
                    &data.2,
                    &mut identifiers,
                    &mut first_identifiers,
                    &mut enum_values
                ),
            }
        ));
    }
    data.0 // skip to next and ignore the data
}

/* Returns:
  0 - nothing
  1 - Is the last node an if?
  2 - Is the last node an else if?
*/
fn is_last_an_if(program: &Vec<Node>) -> u8 {
    let blank = Node::blank();
    let last_node;
    if program.len() == 0 {
        last_node = &blank;
    } else {
        last_node = program.get(program.len() - 1).unwrap_or(&blank);
    }
    if let Some(condition_node) = &last_node.condition_block {
        if condition_node.keyword == "if" {
            return 1;
        }
        if condition_node.keyword == "else if" {
            return 2;
        }
        return 0;
    }
    return 0;
}
