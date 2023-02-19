#include "lexer.hpp"
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
  if (string.size() > 1 && (string.at(0) == '\"' && string.at(string.size()-1)) == '\"' || IsDigitStr(string))
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
  std::vector<Token> output{};
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
    if (!gettingBlock && currentChar.tokenString == "\n") break;
    if (currentChar.tokenString == "{") {
      gettingBlock += 1;
      output.push_back(Token(TokenTypes::BLOCK, std::to_string(gettingBlock), 0, 0));
      // so when you encounter this in the output vector, you know the next stuff in the contents of the block
    }
    if (currentChar.tokenString == "}") {
      gettingBlock -= 1;
      if (gettingBlock == 0) break; // if we collected all the blocks
    }

    output.push_back(currentChar);
  }

  return output;
}

Token Lexer::getToken() {
  // get to word
  // token breakers: ' ', '(', ')', ','

  while (data[position] == ' ' || data[position] == '(' || data[position] == ')' || data[position] == ',')
    position++;

  // get the word
  std::string output = "";
  char currentChar = data[position];
  while (currentChar != ' ' && currentChar != '(' && currentChar != ')' && currentChar != ',') {
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
      column = 0;
    }
    output += currentChar;
    currentChar = data[++position];
    ++column;
  }
  auto tokenType = TokenTypeFromString(output);
  return Token(tokenType, output, line, column);
}

std::vector<Token> Lexer::getTokens() {
  std::vector<Token> tokens;
  for (;position <= data.length();) {
    auto t = getToken();
    if (!t.tokenString.empty()) tokens.push_back(t);
  }
  return tokens;
}