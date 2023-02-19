#include "parser.hpp"

Node::Node() {
}

Parser::Parser(std::string data) {
  auto tokens = lexer.getTokens();

  std::cout << "[";
  for (Token t : tokens) {
    std::cout << t.tokenString << ", ";
  }
  std::cout << "]";

  for (int i = 0; i < tokens.size(); i++) {
    auto token = tokens[i];
    if (token.type == TokenTypes::KEYWORD) {
      Node* n = new Node();
      n->type = T_STATEMENT;
      n->data->s.action = token.tokenString;
      n->data->s.parameters = lexer.getTillEOLOrBlock(tokens, i);
      program.push_back(n);
    } else if ((int)token.type == NodeTypes::T_FUNCTION) {
      auto info = lexer.getTillEOLOrBlock(tokens, i);
      auto identifier = info[0];
    }
  }
}