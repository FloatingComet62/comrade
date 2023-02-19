#include "lexer.hpp"
#include "util.hpp"
#include <iostream>

Token::Token(TokenTypes t, std::string tS, int l, int c) {
  type = t;
  tokenString = tS;
  line = l;
  column = c;
}

bool IsDigit(char c){
	return c >= '0' && c <= '9';
}
bool IsDigitStr(std::string s) {
  for (int i = 0; i < s.size(); i++)
    if (!IsDigit(s.at(i)))
      return false;
  return true;
}
bool IsLetter(char c){
	return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

TokenTypes TokenTypeFromString(std::string string) {
  std::vector<std::string> keywords = { "include", "fun", "return" };
  std::vector<std::string> types = {
    "u4", "u8", "u16", "u32", "u64", "u128",
    "i4", "i8", "i16", "i32", "i64", "i128",
    "f4", "f8", "f16", "f32", "f64", "f128",
    "str", "bool"
  };
  for (std::string keyword : keywords)
    if (string == keyword)
      return TokenTypes::KEYWORD;
  for (std::string type : types)
    if (string == type || string == (type + "[]"))
      return TokenTypes::TYPE;
  if ((string.size() > 1 && (string.at(0) == '\"') && (string.at(string.size()-1)) == '\"') || IsDigitStr(string))
    return TokenTypes::LITERAL;
  if (string.size() > 1 && string.at(0) == '{' && string.at(string.size()-1) == '}')
    return TokenTypes::BLOCK;
  
  return TokenTypes::IDENTIFIER;
}

Lexer::Lexer(std::string src) {
  data = src;
  position = 0;
  line = 1;
  column = 1;
}

std::vector<Token> Lexer::getTillEOLOrBlock(std::vector<Token> tokens, int position) {
  std::function<std::string(Token)> stringToken = [] (Token x) { return x.tokenString; };
  std::vector<Token> output = {};
  int gettingBlock = 0;
  while (true) {
    if (position > tokens.size()) {
      if (gettingBlock) {
        // missing '}'
        std::cout << "A block is unclosed";
        std::exit(1);
      }
      break;
    }
    Token currentChar = tokens[position];
    if (!gettingBlock && currentChar.type == TokenTypes::EOL) break;
    if (currentChar.tokenString == "{") {
      gettingBlock += 1;
    }
    if (currentChar.tokenString == "}") {
      gettingBlock -= 1;
      if (gettingBlock == 0) break; // if we collected all the blocks
    }

    output.push_back(currentChar);
    position++;
  }
  return output;
}

std::tuple<Token, bool> Lexer::getToken() {
  // get to word
  // token breakers: ' ', '(', ')', ','

  while (data[position] == ' ' || data[position] == '\t' || data[position] == '(' || data[position] == ')' || data[position] == ',')
    position++;

  // get the word
  std::string output = "";
  bool EOL = false;
  char currentChar = data[position];
  while (currentChar != ' ' && currentChar != '\t' && currentChar != '(' && currentChar != ')' && currentChar != ',') {
    if (position > data.length()) break;

    // check ->
    if (currentChar == '-' && data[position + 1] == '>') {
      position += 2;
      column += 2;
      break;
    }

    // check =>
    if (currentChar == '=' && data[position + 1] == '>') {
      position += 2;
      column += 2;
      break;
    }

    // check \n
    if (currentChar == '\n') {
      ++line;
      ++position;
      column = 0;
      EOL = true;
      break;
    }
    output += currentChar;
    currentChar = data[++position];
    ++column;
  }
  auto tokenType = TokenTypeFromString(output);
  return std::tuple<Token, bool>{Token(tokenType, output, line, column), EOL};
}

std::vector<Token> Lexer::getTokens() {
  std::vector<Token> tokens;
  for (;position <= data.length();) {
    auto t = getToken();
    auto tt = std::get<0>(t);
    auto eol = std::get<1>(t);
    if (!tt.tokenString.empty()) tokens.push_back(tt);
    if (eol) tokens.push_back(Token(TokenTypes::EOL, "EOL", line, column));
  }
  return tokens;
}