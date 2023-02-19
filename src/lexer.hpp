#pragma once

#include <string>
#include <vector>

enum TokenTypes {
  KEYWORD,
  IDENTIFIER,
  TYPE,
  BLOCK,
  LITERAL,
};


class Token {
  public:
  TokenTypes type;
  std::string tokenString;
  int line;
  int column;
  Token(TokenTypes t, std::string tS, int l, int c);
};

class Lexer {
  public:
  std::string data;
  int position;
  int line;
  int column;
  Lexer(std::string src);
  /*
    Get the next token
  */
  Token getToken();
  /*
    Get's all the tokens from data
  */
  std::vector<Token> getTokens();
  /*
    Get's the content till end of line, if encounters a block, returns the entire tokenized block 

    !ISSUE: curly on the next line will not be recognized by this function yet
  */
  std::vector<Token> getTillEOLOrBlock(std::vector<Token> tokens, int position);
};