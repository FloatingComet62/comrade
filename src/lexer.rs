use crate::{compiler, str_list_to_string_list};

use super::{
    parser::{load, Lexer},
    Node,
};

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
            token_splits: str_list_to_string_list(vec![
                " ", "\r\n", "\n", "\t", "->", ".", "=>", "(", ")", "{", "}", ",", ">=", "<=",
                "==", "+=", "-=", "*=", "/=", "!=", ">", "<", "=", "+", "-", "*", "/", "[", "]",
            ]),
            important_splits: str_list_to_string_list(vec![
                "(", ")", "{", "}", ",", "\r\n", "\n", "->", ">=", "<=", "==", "+=", "-=", "*=",
                "/=", "!=", ">", "<", "=", "=>", "+", "-", "*", "/", "[", "]",
            ]),
            types: str_list_to_string_list(vec![
                "u4", "u8", "u16", "u32", "u64", "u128", "i4", "i8", "i16", "i32", "i64", "i128",
                "f4", "f8", "f16", "f32", "f64", "f128", "str",
            ]),
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
        if string == "\n" {
            return String::from("EOL");
        }
        return string.to_string();
    }
    fn token_splitter(self: &Parser, to_split: &String) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        let mut current_item = String::new();
        let mut getting_string = false;
        let mut getting_exter_c = 0;
        let to_split_chars: Vec<char> = to_split.chars().collect();

        let mut i = 0;
        while i < to_split.len() {
            let item_to_check = to_split_chars[i];
            if item_to_check == '"' {
                getting_string = !getting_string;
            }
            if getting_exter_c == -1 || getting_exter_c > 0 {
                let mut should_add = true;
                if item_to_check == '{' {
                    if getting_exter_c == -1 {
                        getting_exter_c = 0;
                        should_add = false;
                    }
                    getting_exter_c += 1;
                }
                if item_to_check == '}' {
                    getting_exter_c -= 1;
                    if getting_exter_c == 0 {
                        should_add = false;
                    }
                }
                if should_add {
                    current_item += &item_to_check.to_string();
                }
                i += 1;
                continue;
            }
            let (time_to_split, splitter) =
                self.is_split(item_to_check, &current_item, &to_split_chars, i);
            if time_to_split {
                if !getting_string && current_item.len() >= 1 {
                    if current_item == "externC" {
                        getting_exter_c = -1; // -1 to indicate that it's a new start
                    }
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
        output.push(current_item);
        return output;
    }
    pub fn parse(
        self: &Parser,
        print_tokens: bool,
        print_ast: bool,
        print_c_code: bool,
    ) -> Vec<Node> {
        let res = self.token_splitter(&self.data);
        if print_tokens {
            println!("{:?}", res);
        }
        let mut lexer = Lexer::new(res);
        // adding libs here so that they get recognized as identifiers
        // maybe if I make a no std version, I can just make identifiers
        let mut identifiers: Vec<Vec<String>> = lexer.libs.clone();
        let mut first_identifiers: Vec<String> = get_first(&lexer.libs);

        identifiers.append(&mut lexer.keywords.clone());
        for item in get_first(&lexer.keywords) {
            first_identifiers.push(item);
        }

        lexer.program = load(
            &lexer.splitted_text,
            &mut identifiers,
            &mut first_identifiers,
        );
        if print_ast {
            println!("{:?}", lexer.program);
        }

        let c_code = compiler::compiler(&lexer.program);
        if print_c_code {
            println!("{:?}", c_code);
        }

        return lexer.program;
    }
}

pub fn get_first(input: &Vec<Vec<String>>) -> Vec<String> {
    let mut output = vec![];

    for item in input {
        output.push(item[0].clone());
    }

    output
}
