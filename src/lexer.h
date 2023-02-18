#pragma once

#include <string>

enum LaxerTypes {
  KEYWORD,
  EXPRESSION,
  IDENTIFIER,
  TYPE,
  BLOCK
};

enum Errors {
  UnexpectedIdentifier
};


class Token {
  public:
  int Type;
  std::string tokenString;
  int line;
  int column;
  Token(int t, std::string tS, int l, int c);
};

class Lexer {
  public:
  std::string data;
  int position;
  int line;
  int column;
  Lexer(std::string src);
  Token tokenize();
  std::string getToken(int pos);
};