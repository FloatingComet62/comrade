#include "lexer.h"

Token::Token(int t, std::string tS, int l, int c) {
  Type = t;
  tokenString = tS;
  line = l;
  column = c;
}

Lexer::Lexer(std::string src) {
  data = src;
  position = 0;
  line = 1;
  column = 0;
}
std::string Lexer::getToken(int pos) {
  std::string output = "";
  char currentChar = data[pos];
  while (currentChar != ' ' || currentChar != '\n') {
    output += currentChar;
  }

  return output;
}
Token Lexer::tokenize() {
  std::string token = Lexer::getToken(position);
  position++;
  int tokenLine = line;
  int tokenColumn = column++;

  return Token(0, token, line, column);
}