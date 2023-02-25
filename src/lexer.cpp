#include "lexer.hpp"

Token::Token(TokenTypes t, std::string tS, int l, int c) {
  type = t;
  tokenString = tS;
  line = l;
  column = c;
}

bool IsDigit(char c) { return c >= '0' && c <= '9'; }
bool IsDigitStr(std::string s) {
  for (int i = 0; i < s.size(); i++)
    if (!IsDigit(s[i])) return false;
  return true;
}
bool IsLetter(char c) {
  return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

TokenTypes TokenTypeFromString(std::string string) {
  std::vector<std::string> keywords = {"include", "fun", "return"};
  std::vector<std::string> types = {
      "u4",  "u8",   "u16", "u32", "u64", "u128", "i4",  "i8",   "i16", "i32",
      "i64", "i128", "f4",  "f8",  "f16", "f32",  "f64", "f128", "str", "bool"};
  for (std::string keyword : keywords)
    if (string == keyword) return TokenTypes::KEYWORD;
  for (std::string type : types)
    if (string == type || string == (type + "[]")) return TokenTypes::TYPE;
  if ((string.size() > 1 && (string[0] == '\"') &&
       (string[string.size() - 1]) == '\"') ||
      IsDigitStr(string))
    return TokenTypes::LITERAL;
  if (strcmp(string.c_str(), "{") == 0) return TokenTypes::BLOCK;
  if (strcmp(string.c_str(), "}") == 0) return TokenTypes::BLOCK_END;
  if (strcmp(string.c_str(), "(") == 0) return TokenTypes::SYMBOL;
  if (strcmp(string.c_str(), ")") == 0) return TokenTypes::SYMBOL;

  return TokenTypes::IDENTIFIER;
}

Lexer::Lexer(std::string src) {
  data = src;
  position = 0;
  line = 1;
  column = 1;
}

std::tuple<std::vector<Token>, int> Lexer::getTillEOLOrBlock(
    std::vector<Token> tokens, int position, bool needsBlock) {
  std::vector<Token> output = {};
  int newPosition = position;
  int gettingBlock = 0;
  while (true) {
    if (position > tokens.size()) {
      if (gettingBlock) {
        // missing '}'
        std::cout << "A block is unclosed\n";
        std::exit(1);
      }
      break;
    }
    Token currentToken = tokens[position];
    if (!gettingBlock &&  // ! required to be before EOL
        currentToken.type == TokenTypes::EOL) {
      if (
          // we need a block and we see that it's the next token
          needsBlock &&  // ! required before expect
          expect(tokens, Token(TokenTypes::BLOCK, "{", 0, 0), position)) {
        /* don't break */
      } else
        break;
    }
    // ! look at misc/strcmp.txt
    if (strcmp(currentToken.tokenString.c_str(), "{") == 0) {
      gettingBlock += 1;
      output.push_back(Token(TokenTypes::BLOCK, "{", currentToken.line,
                             currentToken.column));

      ++position;
      continue;
    }

    // ! look at misc/strcmp.txt
    if (strcmp(currentToken.tokenString.c_str(), "}") == 0) {
      newPosition = position;
      gettingBlock -= 1;
      output.push_back(Token(TokenTypes::BLOCK_END, "}", currentToken.line,
                             currentToken.column));
      if (gettingBlock == 0) break;  // if we collected all the blocks

      ++position;
      continue;
    }
    output.push_back(currentToken);
    ++position;
  }
  return std::tuple<std::vector<Token>, int>{output, newPosition};
}

std::tuple<Token, bool> Lexer::getToken() {
  // get to word
  // token breakers: ' ', '\t', ','

  auto vecData = vecFromStr(data);

  while (data[position] == ' ' || data[position] == '\t' ||
         data[position] == ',')
    position++;

  // get the word
  std::string output = "";
  bool gettingString = false;

  bool EOL = false;
  char currentChar = data[position];
  while (gettingString ||
         (currentChar != ' ' && currentChar != '\t' && currentChar != ',')) {
    if (position > data.length()) {
      if (gettingString) {
        // missing '"'
        std::cout << "Missing String Terminator\n";
        std::exit(1);
      }
      break;
    }

    if ((currentChar == '(' || currentChar == ')') &&
        output == "") {  // reading '(', ')'
      ++column;
      ++position;
      output += currentChar;
      if (!gettingString) break;
    }
    if (currentChar == '(' ||
        currentChar == ')' &&
            output != "") {  // encountered while reading something else
      if (!gettingString) break;
    }

    // check -> & =>
    if ((currentChar == '-' || currentChar == '=') &&
        expect(vecData, '>', position)) {
      position += 2;
      column += 2;
      if (!gettingString)
        break;  // TODO: maybe return -> or => instead of nothing ??
    }

    // check \n
    if (currentChar == '\n') {
      ++line;
      ++position;
      column = 0;
      EOL = true;
      if (!gettingString) break;
    }

    // check string
    if (currentChar == '"' &&
        !back_expect(vecData, '\\',
                     position)  // "Hello /"Comet/"" => Hello "Comet"
    )
      gettingString = !gettingString;

    output += currentChar;
    currentChar = data[++position];
    ++column;
  }
  auto tokenType = TokenTypeFromString(output);
  return std::tuple<Token, bool>{Token(tokenType, output, line, column), EOL};
}

std::vector<Token> Lexer::getTokens() {
  std::vector<Token> tokens;
  for (; position <= data.length();) {
    auto t = getToken();
    auto tt = std::get<0>(t);
    auto eol = std::get<1>(t);
    if (!tt.tokenString.empty()) tokens.push_back(tt);
    if (eol) tokens.push_back(Token(TokenTypes::EOL, "EOL", line, column));

    // std::cout << tt.tokenString << " ";
    // if (eol) std::cout << "\n";
  }
  tokens.push_back(Token(TokenTypes::EOF, "EOF", line, column));
  return tokens;
}

// ! * SCREW YOU KEBAB CASE