use crate::parser::{load, Lexer, Node};

pub struct Parser {
    pub data: String,
    pub token_splits: Vec<String>,     // splits of token
    pub important_splits: Vec<String>, // split, but keep the splitter
    pub types: Vec<String>,
}

impl Parser {
    pub fn new(data: String) -> Self {
        Self {
            data,
            token_splits: vec![
                String::from(" "),
                String::from("\r\n"),
                String::from("\t"),
                String::from("->"),
                String::from("."),
                String::from("=>"),
                String::from("("),
                String::from(")"),
                String::from("{"),
                String::from("}"),
                String::from(","),
                String::from(">="),
                String::from("<="),
                String::from(">"),
                String::from("<"),
                String::from("="),
                String::from("+"),
                String::from("-"),
                String::from("*"),
                String::from("/"),
                String::from("["),
                String::from("]"),
            ],
            important_splits: vec![
                String::from("("),
                String::from(")"),
                String::from("{"),
                String::from("}"),
                String::from(","),
                String::from("\r\n"),
                String::from("->"),
                String::from(">="),
                String::from("<="),
                String::from(">"),
                String::from("<"),
                String::from("="),
                String::from("=>"),
                String::from("+"),
                String::from("-"),
                String::from("*"),
                String::from("/"),
                String::from("["),
                String::from("]"),
            ],
            types: vec![
                String::from("u4"),
                String::from("u8"),
                String::from("u16"),
                String::from("u32"),
                String::from("u64"),
                String::from("u128"),
                String::from("i4"),
                String::from("i8"),
                String::from("i16"),
                String::from("i32"),
                String::from("i64"),
                String::from("i128"),
                String::from("f4"),
                String::from("f8"),
                String::from("f16"),
                String::from("f32"),
                String::from("f64"),
                String::from("f128"),
                String::from("str"),
            ],
        }
    }
    fn is_split(
        self: &Parser,
        item_to_check: char,
        current_item: &String,
        to_split_chars: &Vec<char>,
        i: usize,
    ) -> (bool, Option<String>) {
        for current_split in &self.token_splits {
            let chars: Vec<char> = current_split.chars().collect();
            if item_to_check == chars[0] {
                // DON'T SPLIT "str[]" INTO "str" "[" "]"
                let mut ci_chars = current_item.chars();
                ci_chars.next_back();
                if current_split == "[" {
                    if self.types.contains(current_item) {
                        return (false, None);
                    }
                }
                if current_split == "]" {
                    if self.types.contains(&ci_chars.as_str().to_string()) {
                        return (false, None);
                    }
                }

                if chars.len() == 1 {
                    return (true, Some(current_split.clone()));
                }
                for j in 1..chars.len() {
                    if chars[j] == to_split_chars[i + j] {
                        return (true, Some(current_split.clone()));
                    }
                }
            }
        }
        return (false, None);
    }
    fn split_print(string: &String) -> String {
        if string == "\r\n" {
            return String::from("EOL");
        }
        return string.to_string();
    }
    fn token_splitter(self: &Parser, to_split: &String) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        let mut current_item = String::new();
        let mut getting_string = false;
        let to_split_chars: Vec<char> = to_split.chars().collect();

        let mut i = 0;
        while i < to_split.len() {
            let item_to_check = to_split_chars[i];
            if item_to_check == '"' {
                getting_string = !getting_string;
            }
            let (time_to_split, splitter) =
                self.is_split(item_to_check, &current_item, &to_split_chars, i);
            if time_to_split {
                if !getting_string && current_item.len() >= 1 {
                    output.push(current_item.clone());
                }
                if let Some(valid_split) = &splitter {
                    if !getting_string && valid_split.len() >= 1 {
                        if self.important_splits.contains(valid_split) {
                            output.push(Parser::split_print(valid_split));
                        }
                    } else {
                        current_item += valid_split;
                    }
                }
                if !getting_string {
                    current_item = String::new();
                }
                i += splitter.unwrap_or(String::new()).len();
                continue;
            }
            current_item += &item_to_check.to_string();
            i += 1;
        }
        return output;
    }
    pub fn parse(self: &Parser, print_tokens: bool, print_ast: bool) -> Vec<Node> {
        let res = self.token_splitter(&self.data);
        if print_tokens {
            println!("{:?}", res);
        }
        let mut lexer = Lexer::new(res);
        lexer.program = load(&lexer.splitted_text);
        if print_ast {
            println!("{:?}", lexer.program);
        }
        return lexer.program;
    }
}
