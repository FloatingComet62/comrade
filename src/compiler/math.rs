use crate::{exit, Math, Operations};

use super::compiler;

pub fn compile(input: &Math, semi_colon_needed: bool) -> String {
    let mut output = String::new();
    output += &compiler(&input.lhs, String::new(), false, false);
    output += match input.operation {
        Operations::ADD => "+",
        Operations::ADDEQT => "+=",
        Operations::DIV => "/",
        Operations::DIVEQT => "/=",
        Operations::EQGR => ">=",
        Operations::EQLT => "<=",
        Operations::MUL => "*",
        Operations::MULEQT => "*=",
        Operations::NEQ => "!=",
        Operations::NULL => exit("Unknown operation", None),
        Operations::SUB => "-",
        Operations::SUBEQT => "-=",
        Operations::GR => ">",
        Operations::LT => "<",
        Operations::EQT => "==",
        Operations::EQ => "=",
    };
    output += &compiler(&input.rhs, String::new(), false, false);
    if semi_colon_needed {
        output += ";";
    }
    output
}
