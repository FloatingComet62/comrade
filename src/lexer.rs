use crate::{TokenTypes, Token};

use super::exit;

#[derive(Debug)]
pub struct Lexer {
  data: String,
  position: i32,
  line: i32,
  column: i32,
}

fn is_digit_str(s: &str) -> bool {
  for c in s.chars() {
    if !c.is_digit(10) { return false; }
  }
  return true;
}

fn token_type_from_string(string: &str) -> TokenTypes {
  let keywords: Vec<&str> = vec!["include", "fun", "return"];
  let types: Vec<&str> = vec![
      "u4",  "u8",   "u16", "u32", "u64", "u128", "i4",  "i8",   "i16", "i32",
      "i64", "i128", "f4",  "f8",  "f16", "f32",  "f64", "f128", "str", "bool"];
  for keyword in keywords {
    if string == keyword { return TokenTypes::KEYWORD; }
  }
  for t in types {
    if string == t || string == (t.to_owned() + "[]") { return TokenTypes::TYPE; }
  }
  let chars: Vec<char> = string.chars().collect();
  let first_char = chars.get(0).unwrap_or(&'\0');
  let last_char = chars.get(chars.len() - 1).unwrap_or(&'\0');
  if string.len() > 1 && (first_char == &'\"') &&
       (last_char == &'\"') ||
      is_digit_str(string) {
        return TokenTypes::LITERAL;
      }
  if string == "{" { return TokenTypes::BLOCK; }
  if string == "}" { return TokenTypes::BLOCKEND; }
  if string == "(" { return TokenTypes::SYMBOL; }
  if string == ")" { return TokenTypes::SYMBOL; }

  return TokenTypes::IDENTIFIER;
}

impl Lexer {
  pub fn new(src: String) -> Self {
    Self {
      data: src,
      position: 0,
      line: 1,
      column: 1
    }
  }
  /* Get's the next token
     Returns:
       Token Found
       EOL -> End of Line
  */
  pub fn get_token(self: &mut Lexer) -> (Token, bool) {
  // get to word
  // token breakers: ' ', '\t', ','

  let vec_data: Vec<char> = self.data.chars().collect();

  let raw_current_char = vec_data.get(self.position as usize);
  match raw_current_char {
    Some(current_char) => {
      while current_char == &' ' || current_char == &'\t' || current_char == &',' {
        self.position += 1;
        self.column += 1;
      }
    }
    None => {
      return (Token {
        t_type: TokenTypes::EOF,
        token_string: "EOF".to_string(),
        column: self.column,
        line: self.line
      }, false)
    }
  }

  // get the word
  let mut output = String::new();
  let mut getting_string = false;

  let mut eol = false;
  loop {
    let raw_current_char = vec_data.get(self.position as usize);
    match raw_current_char {
      Some(current_char) => {
        if current_char == &' ' || current_char == &'\t' || current_char == &',' { break }
      
        // reading '(', ')'
        if (current_char == &'(' || current_char == &')') &&  output == "" {
          self.column += 1;
          self.position += 1;
          output += &current_char.to_string();
          if !getting_string { break; }
        }
        // encountered while reading something else
        if current_char == &'(' || current_char == &')' && output != "" {
          if !getting_string { break; }
        }
      
        // check -> & =>
        if (current_char == &'-' || current_char == &'=') &&
            Lexer::expect(&vec_data, '>', self.position as usize) {
          self.position += 2;
          self.column += 2;
          if !getting_string {
            break;  // TODO: maybe return -> or => instead of nothing ??
          }
        }
      
        // check \n
        if current_char == &'\n' {
          self.line += 1;
          self.position += 1;
          self.column = 0;
          eol = true;
          if !getting_string { break; }
        }
      
        // check string
        if current_char == &'"' &&
          !Lexer::back_expect(&vec_data, '\\', self.position as usize) {
          // "Hello /"Comet/"" => Hello "Comet"
          getting_string = !getting_string;
        }
      
        output += &current_char.to_string();
        self.position += 1;
        self.column += 1;
      }
      None => {
        if getting_string {
          // missing '"'
          exit("Missing String Terminator", None);
        }
      }
    }
  }
  let token_type = token_type_from_string(&output);
  return (Token {
    t_type: token_type,
    token_string: output.to_string(),
    line: self.line,
    column: self.column
  }, eol);
  }
  // Get's all the tokens from data
  pub fn get_tokens(self: &mut Lexer) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    while self.position <= self.data.len() as i32 {
      let t = self.get_token();
      let tt = t.0;
      let eol = t.1;
      if !tt.token_string.is_empty() { tokens.push(tt); }
      if eol { tokens.push(Token{
        t_type: TokenTypes::EOL,
        token_string: "EOL".to_string(),
        line: self.line,
        column: self.column
      }); }
    }
    tokens.push(Token {
      t_type: TokenTypes::EOF,
      token_string: "EOF".to_string(),
      line: self.line,
      column: self.column
    });
    return tokens;
  }
  // Returns if the next token is the argument
  pub fn expect<T: std::cmp::PartialEq>(d: &Vec<T>, expected_value: T, current_position: usize) -> bool {
    match d.get(current_position + 1) {
      Some(x) => x == &expected_value,
      None => false
    }
  }
  // Returns if the previous token is the argument
  pub fn back_expect<T: std::cmp::PartialEq>(d: &Vec<T>, expected_value: T, current_position: usize) -> bool {
    match d.get(current_position - 1) {
      Some(x) => x == &expected_value,
      None => false
    }
  }

  /*
    Get's the content till end of line, if encounters a block, returns the
    entire tokenized block
    needsBlock -> if false, it will not check for a block in the next line
    Returns:
      Tokens
      new position for skipping the block
    ! ISSUE: curly on the next line will not be recognized by this function yet
  */
  pub fn get_till_eol_or_block(tokens: &Vec<Token>, position: i32, needs_block: bool)
    -> (Vec<Token>, i32) {
    let mut output: Vec<Token> = vec![];
    let mut new_position = position;
    let mut getting_block = 0;

    loop {
      if position > tokens.len() as i32 {
        if getting_block > 0 {
          // missing '}'
          exit("A block is unclosed", None);
        }
      }
      let current_token = &tokens[position as usize];
      if getting_block == 0 && // ! required before EOL
        current_token.t_type == TokenTypes::EOL {
          if 
            // we need a block and we see that it's the next token
            needs_block &&  // ! required before expect
            Lexer::expect(&tokens, Token {
              t_type: TokenTypes::BLOCK,
              token_string: "{".to_string(),
              column: 0,
              line: 0
            }, position as usize) {
          /* don't break */
        } else { break; };
      }

      if current_token.token_string == "{" {
        getting_block += 1;
        output.push(Token {
          t_type: TokenTypes::BLOCK,
          token_string: "{".to_string(),
          line: current_token.line,
          column: current_token.column
        });
      }
      if current_token.token_string == "}" {
        new_position = position;
        getting_block -= 1;
        output.push(Token {
          t_type: TokenTypes::BLOCKEND,
          token_string: "}".to_string(),
          line: current_token.line,
          column: current_token.column
        });

        if getting_block == 0 {
          // if we have collected all the blocks
          break;
        }
      }
    }
    return (output, new_position)
  }
}