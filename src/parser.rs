use crate::nodes::{self, NodeInterferace};
use crate::{str_list_to_string_list, type_from_str, Expression, Literal, Node, NodeData, Types};

#[derive(Clone)]
pub struct ParserData {
    pub identifier: Vec<Vec<String>>,
    pub enum_values: Vec<Vec<String>>,
    pub struct_data: Vec<Vec<String>>,
}
impl ParserData {
    pub fn new(std: bool) -> Self {
        if !std {
            return Self {
                identifier: vec![],
                enum_values: vec![],
                struct_data: vec![],
            };
        }
        Self {
            identifier: {
                let mut output = vec![];
                for keywords in KEYWORDS.iter() {
                    let item: Vec<String> = keywords.iter().map(|x| x.to_string()).collect();
                    output.push(item);
                }
                for keywords in LIB.iter() {
                    let item: Vec<String> = keywords.iter().map(|x| x.to_string()).collect();
                    output.push(item);
                }
                output
            },
            enum_values: vec![],
            struct_data: vec![],
        }
    }
}

pub const LIB: [[&str; 2]; 2] = [["std", "io"], ["std", "math"]];
pub const KEYWORDS: [[&str; 1]; 2] = [["default"], ["NULL"]];

pub enum Mode {
    Or,
    And,
}

pub fn has(data: &[String], things: Vec<&str>, mode: Mode) -> bool {
    match mode {
        Mode::And => {
            for thing in things {
                if !data.contains(&thing.to_string()) {
                    return false;
                }
            }
            true
        }
        Mode::Or => {
            for thing in things {
                if data.contains(&thing.to_string()) {
                    return true;
                }
            }
            false
        }
    }
}

#[derive(Clone)]
pub struct Parser {
    pub program: Vec<Node>,
    pub keywords: Vec<Vec<String>>,
    pub libs: Vec<Vec<String>>,
    pub input: Vec<String>,
    pub parser_data: ParserData,
    pub math_tokens: [&'static str; 17],
    pub math_ops: [&'static str; 15],
}
impl Parser {
    pub fn new(input: Vec<String>, parser_data: ParserData) -> Self {
        Self {
            program: vec![],
            keywords: KEYWORDS
                .iter()
                .map(|f| {
                    f.iter()
                        .map(|x| x.to_string().to_string())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>(),
            libs: LIB
                .iter()
                .map(|f| {
                    f.iter()
                        .map(|x| x.to_string().to_string())
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>(),
            math_tokens: [
                "+", "-", "*", "/", "+=", "-=", "*=", "/=", "=", "==", "!=", ">", "<", ">=", "<=",
                "(", ")",
            ],
            math_ops: [
                "+", "-", "*", "/", "==", "=", ">", "<", "<=", ">=", "!=", "+=", "-=", "*=", "/=",
            ],
            input,
            parser_data,
        }
    }

    ///
    /// Returns:
    /// new index
    /// content till token
    /// block found
    /// & math block found
    ///
    pub fn get_till_token_or_block_and_math_block(
        &self,
        token: &str,
        i: usize,
    ) -> (usize, Vec<String>, Vec<String>, bool, Vec<String>) {
        let mut output: Vec<String> = vec![];
        let mut block: Vec<String> = vec![];
        let mut math_block = vec![];
        let mut got_block = false;
        let mut getting_block = 0;
        let mut getting_function_call = 0;
        let mut is_comment = false;
        let mut j = i + 1;

        while j < self.input.len() {
            let text = &self.input[j];
            // ! must be first
            if text == "//" {
                is_comment = true;
            }
            if text == "EOL" {
                is_comment = false;
            }
            if is_comment {
                j += 1;
                continue;
            }
            let operator = self.is_math(text.to_string());
            if operator {
                math_block.push(text.to_string());
            }

            if text == "{" {
                getting_block += 1;
            }
            if text == "(" && token != "(" {
                // we are not getting the (, then get the entire function
                getting_function_call += 1;
            }
            if text == ")" {
                getting_function_call -= 1;
            }
            if text == token && getting_block == 0 && getting_function_call == 0 {
                break;
            }
            if getting_block > 0 {
                block.push(text.to_string());
            } else {
                output.push(text.to_string());
            }
            // ! must be last
            if text == "}" {
                got_block = true;
                getting_block -= 1;
                if getting_block == 0 && getting_function_call == 0 {
                    // got a random block?
                    // IT PROBABLY WAS IMPORTANT OR SOMETHING, ISN'T IT
                    // stop everything, just return
                    break;
                }
            }

            j += 1;
        }
        (j, output, block, got_block, math_block)
    }

    fn is_math(&self, token: String) -> bool {
        has(&[token], self.math_tokens.to_vec(), Mode::Or)
    }
    fn is_math_vec(&self, token: &Vec<String>) -> bool {
        has(token, self.math_tokens.to_vec(), Mode::Or)
    }

    fn list_check(&self, idenf: &mut Vec<String>, input: &Vec<String>, i: usize) -> Option<()> {
        if let Some(first) = input.get(i + 1) {
            if !(first == "[") {
                return None;
            }
        }
        if let Some(second) = input.get(i + 3) {
            if !(second == "]") {
                return None;
            }
        }
        if let Some(middle) = input.get(i + 2) {
            let chars: Vec<char> = middle.chars().collect();
            if !chars[0].is_numeric() {
                return None;
            }
            // it is a list indexing
            idenf.append(&mut str_list_to_string_list(vec!["[", middle, "]"]));
            return Some(());
        }
        return None;
    }

    fn identifier_check(
        &self,
        program: &mut Vec<Node>,
        i: usize,
        iden: &Vec<String>,
    ) -> Option<usize> {
        let mut identifer = true;
        let mut i_adder: usize = 0;
        for (k, id) in iden.iter().enumerate() {
            if let Some(to_check) = self.input.get(i + k) {
                println!("{}", to_check);
                if to_check != id.as_str() {
                    identifer = false;
                }
            } else {
                // we reached the end of the file
                break;
            }
        }
        if !identifer {
            return None;
        }
        let mut idenf = iden.clone();
        if self.list_check(&mut idenf, &self.input, i).is_some() {
            i_adder += 3;
        }
        program.push(Node::new(
            NodeData::Expression(Expression { expr: idenf }),
            0,
            0,
        ));
        i_adder += iden.len();
        Some(i_adder)
    }

    fn enum_check(&self, program: &mut Vec<Node>, text: String, enum_val: &Vec<String>, i: usize) {
        if !(text == enum_val[0]) {
            return;
        }
        for (j, val) in enum_val.iter().enumerate() {
            if !(&self.input[i + j] == val) {
                continue;
            }
            program.push(Node::new(
                NodeData::Literal(Literal {
                    literal: enum_val.join("_"),
                    l_type: Types::I32,
                }),
                0,
                0,
            ))
        }
    }

    /// Generate the AST
    /// * `identifiers` - Identifiers to look out for
    /// * `enum_values` - Enum Values to look out for
    /// * `struct_data` - Structs to look out for
    pub fn load(&mut self) {
        let mut program = vec![];
        // previous token
        let mut previous_text;
        let mut text = &String::new();
        let mut i = 0;
        while i < self.input.len() {
            let data = self.get_till_token_or_block_and_math_block("EOL", i);
            previous_text = text.clone();
            text = &self.input[i];

            if text == "//" {
                i += data.0;
                continue;
            }

            // skip EOLs
            if text == "EOL" || text == "(" || text == ")" {
                i += 1;
                continue;
            }

            let is_math = self.is_math_vec(&data.1);
            let is_math_op = has(&data.1, self.math_ops.to_vec(), Mode::Or);

            // check if no math
            if !is_math {
                // identifier check
                for iden in self.parser_data.identifier.iter() {
                    if !(text == &iden[0]) {
                        continue;
                    }
                    match self.identifier_check(&mut program, i, &iden) {
                        None => {}
                        Some(x) => {
                            i += x;
                            break;
                        }
                    }
                }
                // enum values
                for enum_val in self.parser_data.enum_values.iter() {
                    self.enum_check(&mut program, text.to_string(), enum_val, i);
                }
            }

            let bm = nodes::booleans::BooleanManager::new();
            let em = nodes::enum_expr::EnumManager::new();
            let exm = nodes::extern_c::ExternCManager::new();
            let fcm = nodes::function_call::FunctionCallManager::new(data.1.clone());
            let fm = nodes::function::FunctionManager::new();
            let ifm = nodes::if_expr::IfElseManager::new();
            let lm = nodes::literal::LiteralManager::new({
                if text.chars().next().unwrap_or('\0') == '\"' {
                    Types::Str
                } else if {
                    let mut output = false;
                    if text.len() >= 2
                        && text.chars().next().expect("impossible") == '-'
                        && text.chars().nth(1).expect("impossible").is_numeric()
                    {
                        output = true;
                    }
                    if !output {
                        // there is only 1 previous true assignment for output, so if output is true, then that means it is -ve
                        output = text.chars().next().unwrap_or('\0').is_numeric();
                    }

                    output
                } {
                    Types::I32
                } else if type_from_str(text) != Types::None {
                    Types::Type
                } else {
                    Types::None
                }
            });
            let mm = nodes::match_expr::MatchManager::new();
            let mam = nodes::math::MathManager::new(data.1.clone(), data.4.clone(), is_math_op);
            let sm = nodes::statement::StatementManager::new();
            // let stm = nodes::statement::StatementManager::new();
            let strm = nodes::struct_expr::StructManager::new();
            let vam = nodes::variable_assignment::VariableAssignmentManager::new(text == "const");
            let wm = nodes::while_expr::WhileManager::new();

            // order -> exm, vam, sm, bm, ifm, wm, strm, em, fm, mam, lm, mm, fcm

            macro_rules! parse {
                ($var: ident) => {
                    $var.parser(
                        self.clone(),
                        &mut program,
                        data.clone(),
                        text,
                        &previous_text,
                        &self.input,
                        &mut i,
                        &mut self.parser_data,
                    );
                };
            }

            if exm.check(text.to_string()) {
                parse!(exm);
            } else if vam.check(text.to_string()) {
                parse!(vam);
            } else if sm.check(text.to_string()) {
                parse!(sm);
            } else if bm.check(text.to_string()) {
                parse!(bm);
            } else if ifm.check(text.to_string()) {
                parse!(ifm);
            } else if wm.check(text.to_string()) {
                parse!(wm);
            } else if strm.check(text.to_string()) {
                parse!(strm);
            } else if em.check(text.to_string()) {
                parse!(em);
            } else if fm.check(text.to_string()) {
                parse!(fm);
            } else if mam.check(text.to_string()) {
                parse!(mam);
            } else if lm.check(text.to_string()) {
                parse!(lm);
            } else if mm.check(text.to_string()) {
                parse!(mm);
            } else if fcm.check(text.to_string()) {
                // also, it's a function call when there is no fun
                parse!(fcm);
            }
            i += 1;
        }

        self.program = program;
    }
}
