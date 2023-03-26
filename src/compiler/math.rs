use crate::{exit, Math, Operations};

use super::compiler;

pub fn compile(semi_colon_needed: bool, input: &mut Math) -> String {
    let mut output = String::new();
    output += &compiler(&mut input.lhs, String::new(), false);
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
    };
    output += &compiler(&mut input.rhs, String::new(), false);
    if semi_colon_needed {
        output += ";";
    }
    output
}
