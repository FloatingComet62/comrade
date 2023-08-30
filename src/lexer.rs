use crate::str_list_to_string_list;

/// # Lexer
/// Lexer will split the code into tokens
///
/// * `data` - The code that needs to be tokenized
/// * `token_splits` - The characters that seperate 2 tokens from each other
/// * `important_splits` - token_splits that themselves are important to be present between the tokens<br><br>
/// `fun main(args1 -> type1)`<br>
/// `"fun" "main" "(" "arg1" "->" "type1" ")"`<br>
/// <br><br>
/// * `types` - all the types in the language
pub struct Lexer {
    pub data: String,
    pub token_splits: Vec<String>,
    pub important_splits: Vec<String>,
    pub types: Vec<String>,
}

impl Lexer {
    pub fn new(data: String) -> Self {
        Self {
            data,
            token_splits: str_list_to_string_list(vec![
                "//", " ", "\r\n", "\n", "\t", "->", ".", "=>", "(", ")", "{", "}", ",", ">=",
                "<=", "==", "+=", "-=", "*=", "/=", "!=", ">", "<", "=", "+", "-", "*", "/", "[",
                "]",
            ]),
            important_splits: str_list_to_string_list(vec![
                "//", "(", ")", "{", "}", ",", "\r\n", "\n", "->", ">=", "<=", "==", "+=", "-=",
                "*=", "/=", "!=", ">", "<", "=", "=>", "+", "-", "*", "/", "[", "]",
            ]),
            types: str_list_to_string_list(vec![
                "u4", "u8", "u16", "u32", "u64", "u128", "i4", "i8", "i16", "i32", "i64", "i128",
                "f4", "f8", "f16", "f32", "f64", "f128", "str",
            ]),
        }
    }
    /// Is it time to split the code?
    /// * `item_to_check` - The item we are checking
    /// * `current_item` - The
    fn is_split(
        self: &Lexer,
        item_to_check: char,
        current_item: &String,
        to_split_chars: &[char],
        i: usize,
    ) -> (bool, Option<String>) {
        for current_split in &self.token_splits {
            let chars: Vec<char> = current_split.chars().collect();
            if item_to_check == chars[0] {
                // DON'T SPLIT "str[]" INTO "str" "[" "]"
                let mut ci_chars = current_item.chars();
                ci_chars.next_back();
                if current_split == "[" && self.types.contains(current_item) {
                    return (false, None);
                }
                if current_split == "]" && self.types.contains(&ci_chars.as_str().to_string()) {
                    return (false, None);
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
        (false, None)
    }
    /// any special printing??
    fn split_print(string: &String) -> String {
        if string == "\r\n" {
            return String::from("EOL");
        }
        if string == "\n" {
            return String::from("EOL");
        }
        string.to_string()
    }

    // split code into tokens
    pub fn token_splitter(self: &Lexer) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        // Item we are working on
        let mut current_item = String::new();
        // Are we getting a string
        let mut getting_string = false;
        // Are we getting externC
        let mut getting_exter_c = 0;
        let to_split_chars: Vec<char> = self.data.chars().collect();

        let mut i = 0;
        while i < self.data.len() {
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
            if !time_to_split {
                current_item += &item_to_check.to_string();
                i += 1;
                continue;
            }
            if !getting_string && !current_item.is_empty() {
                if current_item == "externC" {
                    getting_exter_c = -1; // -1 to indicate that it's a new start
                }
                output.push(current_item.clone());
            }
            if let Some(valid_split) = &splitter {
                if to_split_chars.len() > i + 1
                    && to_split_chars[i + 1].is_numeric()
                    && valid_split == "-"
                {
                    current_item += "-";
                    // don't split, it's a negative number
                    i += 1;
                    continue;
                }
                if !getting_string && !valid_split.is_empty() {
                    if self.important_splits.contains(valid_split) {
                        output.push(Lexer::split_print(valid_split));
                    }
                } else {
                    current_item += valid_split;
                }
            }
            if !getting_string {
                current_item = String::new();
            }
            i += splitter.unwrap_or_default().len();
        }
        output.push(current_item);
        output
    }
}
