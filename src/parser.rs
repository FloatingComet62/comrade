use crate::{
  lexer::Lexer, Token, Node, TokenTypes, NodeTypes,
  Statement, exit, Argument, type_from_str, Types, Function, ArgumentNLiteral, Literal, FunctionCall
};

/*
  AST Generator
*/
pub struct Parser {
  pub data: String,
  pub tokens: Vec<Token>,
  pub lexer: Lexer,
  pub program: Vec<Node>,
}

impl Parser {
  pub fn new(data: String) -> Self {
    let mut l = Lexer::new(data.clone());
    let t = l.get_tokens();
    Self {
      data,
      lexer: l,
      tokens: t.clone(),
      program: Parser::parse(t)
    }
  }
  pub fn parse(to_parse: Vec<Token>) -> Vec<Node> {
    let eof = &Token {
      t_type: TokenTypes::EOF,
      token_string: "EOF".to_string(),
      line: -1,
      column: -1
    };
    let mut output: Vec<Node> = vec![];
    let mut i = 0;
    while i < to_parse.len() {
      let token = to_parse.get(i).unwrap_or(eof);
      if token.token_string == "include" || token.token_string == "return" {
        // include and return have the same implementation for now
        let response = Parser::include(&to_parse, token, i);
        i = response.1;
        output.push(response.0);
      } else if token.token_string == "fun" {
        let response = Parser::fun(&to_parse, token, i);
        i = response.1;
        output.push(response.0);
      } else if token.token_string == "(" {
        let response = Parser::fun_call(&to_parse, token, i);
        i = response.1;
        output.push(response.0);
      }
    }
  
    return output;
  }
  /*
    Returns:
      Node -> Node to add
      position -> position to skip to after the function
  */
  fn include(to_parse: &Vec<Token>, token: &Token, i: usize) -> (Node, usize) {
    let mut n = Node::new(NodeTypes::TSTATEMENT, token.token_string.clone());
    let res = Lexer::get_till_eol_or_block(to_parse, i as i32, false);
    let s = Statement {
      action: token.token_string.clone(),
      parameters: res.0
    };
    n.data.set_statement(s);
  
    return(n, res.1 as usize)
  }
  /*
  Returns:
    Node -> Node to add
    position -> position to skip to after the function
  */
  fn fun(to_parse: &Vec<Token>, token: &Token, i: usize) -> (Node, usize) {
    let res = Lexer::get_till_eol_or_block(to_parse, i as i32, true);
    let data = res.0;
    // get the entire identifier
    let mut iter = i;
    let mut identifier: Vec<Token> = vec![];
    loop {
      let raw_current_char = to_parse.get(iter);
      match raw_current_char {
        Some(current_char) => {
          if current_char.t_type == TokenTypes::SYMBOL { break; }
          identifier.push(current_char.clone());
          iter += 1;
        }
        None => exit("Missing the function paramters\n", None)
      }
    }
  
    let raw_block_begin = data.iter().position(|x| *x == Token {
      t_type: TokenTypes::BLOCK,
      token_string: "{".to_string(),
      line: 0,
      column: 0
    });
    let block_begin;
    match raw_block_begin {
      Some(bb) => block_begin = bb,
      None => exit("Missing the function block\n", None)
    };

    let raw_block_end = data.iter().position(|x| *x == Token {
      t_type: TokenTypes::BLOCKEND,
      token_string: "}".to_string(),
      line: 0,
      column: 0
    });
    let block_end;
    match raw_block_begin {
      Some(bb) =>  block_end = bb,
      // Unreachable
      None => exit("Missing the function block terminator\n", None)
    };

    // get the token before "{" -> which is the return type
    let return_type_index = block_begin - 1;
    let raw_return_type = data.get(return_type_index);
    let return_type;
    match raw_return_type {
      Some(rt) => return_type = rt.clone(),
      None => exit("Missing Function return type", None)
    };
    let block = &data[block_begin + 1..block_end - 1];

    let raw_arg_start = data.iter().position(|x| *x == Token {
      t_type: TokenTypes::SYMBOL,
      token_string: "(".to_string(),
      line: 0,
      column: 0
    });
    let arg_start;
    match raw_arg_start {
      Some(at) => arg_start = at,
      None => exit("Missing '('", None)
    };
    let raw_arg_end = data.iter().position(|x| *x == Token {
      t_type: TokenTypes::SYMBOL,
      token_string: ")".to_string(),
      line: 0,
      column: 0
    });
    let arg_end;
    match raw_arg_end {
      Some(ae) => arg_end = ae,
      None => exit("Missing ')'", None)
    };

    let block = &data[block_begin + 1..block_end - 1];
  
    let raw_arguments = &data[arg_start..arg_end];
    let mut arguments: Vec<Argument> = vec![];
    let mut current_argument = Argument {
      identifier: Token { t_type: TokenTypes::KEYWORD, token_string: "TEMP".to_string(), line: 0, column: 0 },
      a_type: Types::I32
    };
    for arg in raw_arguments {
      if arg.t_type == TokenTypes::TYPE {
        current_argument.a_type = type_from_str(&arg.token_string);
        arguments.push(current_argument.clone());
      }
      if arg.t_type == TokenTypes::IDENTIFIER {
        current_argument.identifier = arg.clone();
      }
    }
    let mut n = Node::new(NodeTypes::TFUNCTION, token.token_string.clone());
    n.data.set_function(Function {
      identifier: identifier,
      return_type: return_type,
      arguments: arguments,
      nodes: Parser::parse(block.to_vec())
    });

    return (n, res.1 as usize);
  }
  /*
  Returns:
    Node -> Node to add
    position -> position to skip to after the function
  */
  fn fun_call(to_parse: &Vec<Token>, token: &Token, i: usize) -> (Node, usize) {
    // find the entire identifier
    let mut iter = i;
    let mut identifier: Vec<Token> = vec![];
    loop {
      let raw_current_char = to_parse.get(iter);
      match raw_current_char {
        Some(current_char) => {
          if current_char.t_type == TokenTypes::SYMBOL { break; }
          identifier.push(current_char.clone());
          iter -= 1;
        }
        None => break
      }
    }
    // flip it to correct order
    identifier.reverse();

    iter = i;
    loop {
      let raw_current_char = to_parse.get(iter);
      match raw_current_char {
        Some(current_char) => {
          if current_char.token_string == ")" { break; }
          iter += 1;
        }
        None => exit("Missing the function parameter end", None)
      }
    }

    let index_of_argument_end = iter;
    let raw_arguments = &to_parse[i..index_of_argument_end];
    let arguments: Vec<ArgumentNLiteral> = vec![];
    let getting_string = false;
    for current_argument in raw_arguments {
      let mut anl = ArgumentNLiteral {
        argument: None,
        literal: None
      };
      let raw_first_char = current_argument.token_string.chars().nth(0);
      match raw_first_char {
        Some(first_char) => {
          if first_char == '\"' {
            anl.literal = Some(Literal {
              l_type: Types::Str,
              literal: Token {
                t_type: TokenTypes::LITERAL,
                token_string: current_argument.token_string.clone(),
                line: current_argument.line,
                column: current_argument.column
              }
            });
          } else if first_char.is_digit(10) {
            anl.literal = Some(Literal {
              l_type: Types::I32,
              literal: Token {
                t_type: TokenTypes::LITERAL,
                token_string: current_argument.token_string.clone(),
                line: current_argument.line,
                column: current_argument.column
              }
            })
          } else {
            anl.argument = Some(Argument {
              identifier: Token {
                t_type: TokenTypes::IDENTIFIER,
                token_string: current_argument.token_string.clone(),
                line: current_argument.line,
                column: current_argument.column
              },
              a_type: Types::Str
              // TODO: look at the AST and find the identifier, and put type of the identifier here
            })
          }
        }
        None => {}
      }
    }

    let mut main = String::new();
    for arg in &arguments {
      match &arg.literal {
        Some(l) => main += &l.literal.token_string,
        None => {}
      }
      match &arg.argument {
        Some(a) => main += &a.identifier.token_string,
        None => {}
      }
    }

    let mut n = Node::new(NodeTypes::TFUNCTIONCALL, main);
    n.data.set_function_call(FunctionCall { identifier, arguments });

    return (n, i);

  }
}