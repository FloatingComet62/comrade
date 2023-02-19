#include "parser.hpp"
#include "util.hpp"

Node::Node() {}

// Function to slice a given vector
// from range X to Y
std::vector<Token> slicing(std::vector<Token>& arr, int X, int Y) {
  // Starting and Ending iterators
  auto start = arr.begin() + X;
  auto end = arr.begin() + Y + 1;
 
  // To store the sliced vector
  std::vector<Token> result(Y - X + 1);
 
  // Copy vector using copy function()
  copy(start, end, result.begin());
 
  // Return the final sliced vector
  return result;
}

Parser::Parser(std::string data) {
  std::function<std::string(Token)> stringToken = [] (Token x) { return x.tokenString; };
  std::function<std::string(Node*)> stringNode = [] (Node* x) { return x->main; };

  lexer = Lexer(data);
  auto tokens = lexer.getTokens();
  print_vec(tokens, stringToken);
  for (int i = 0; i < tokens.size(); i++) {
    auto token = tokens.at(i);
    if (token.tokenString == "include" || token.tokenString == "return") {
      Node* n = new Node();
      n->type = NodeTypes::T_STATEMENT;
      n->data->s.action = token.tokenString;
      n->main = token.tokenString;
      n->data->s.parameters = lexer.getTillEOLOrBlock(tokens, i);
      program.push_back(n);
    } else if (token.tokenString == "fun") {
      auto data = lexer.getTillEOLOrBlock(tokens, i);
      auto identifier = data.at(1);

      // iterate and find "{"
      auto it = find(data.begin(), data.end(), "{");
      if (it == data.end()) {
        std::cout << "Missing the function block";
        std::exit(1);
      }
      // get the token before "{"
      auto index = --it - data.begin();
      auto returnType = data.at(index);

      auto raw_arguments = slicing(data, 2, index-1);
      std::vector<Argument> arguments;
      for (int i = 0; i < raw_arguments.size(); i++) {
        auto arg = raw_arguments.at(i);

        if (arg.type == TokenTypes::TYPE) {
          arguments.at(arguments.size()-1).type = arg;
        }
        if (arg.type == TokenTypes::IDENTIFIER) {
          Argument* a = new Argument();
          a->identifier = arg;

          arguments.push_back(*a);
        }
      }

      Node* n = new Node();
      n->type = NodeTypes::T_FUNCTION;
      n->main = token.tokenString;
      n->data->f.arguments = arguments;
      n->data->f.identifier = identifier;
      n->data->f.returnType = returnType;
      program.push_back(n);
    }
  }
  print_vec(program, stringNode);
}