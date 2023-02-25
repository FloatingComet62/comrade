#pragma once

#include <string>
#include <tuple>

#include "util.hpp"

bool IsDigit(char c);
bool IsDigitStr(std::string s);
bool IsLetter(char c);

class Token {
 public:
  TokenTypes type;
  std::string tokenString;
  int line;
  int column;
  Token(TokenTypes t, std::string tS, int l, int c);
  Token() { memset(this, 0, sizeof(Token)); }

  // the point is that you don't compare line and column
  friend bool operator==(const Token& lhs, const Token& rhs) {
    return (lhs.tokenString == rhs.tokenString) && (lhs.type == rhs.type);
  }
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

    Returns:
      Token found
      Symbol token found
      EOL -> End of Line
  */
  std::tuple<Token, bool> getToken();
  /*
    Get's all the tokens from data
  */
  std::vector<Token> getTokens();
  /*
    Returns if the next token is the argument
  */
  template <typename T>
  bool expect(std::vector<T> d, T expectedValue, int currentPosition) {
    if (currentPosition == d.size() - 1) return false;  // bounds
    return d[currentPosition + 1] == expectedValue;
  }
  template <typename T>
  bool back_expect(std::vector<T> d, T expectedValue, int currentPosition) {
    if (currentPosition == 0) return false;  // bounds
    return d[currentPosition - 1] == expectedValue;
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
  std::tuple<std::vector<Token>, int> getTillEOLOrBlock(
      std::vector<Token> tokens, int position, bool needsBlock);
};