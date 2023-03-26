use crate::Expression;

pub fn compile(input: &Expression) -> String {
    let mut output = String::new();
    let mut list_indexing = false;
    for (i, item) in input.expr.iter().enumerate() {
        let blank = String::new();
        let next = input.expr.get(i + 1).unwrap_or(&blank);
        if next == "[" {
            list_indexing = true;
        }

        output += item;
        if !list_indexing {
            output += ".";
        }
        if item == "]" {
            list_indexing = false;
        }
    }
    output
}
