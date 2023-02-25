#include "parser.hpp"

Node::Node() { malloc(sizeof(Node)); }
NodeData::NodeData() { memset(this, 0, sizeof(NodeData)); }
Argument_N_Literal::Argument_N_Literal() {
  memset(this, 0, sizeof(Argument_N_Literal));
}
Argument::Argument() { memset(this, 0, sizeof(Argument)); }

// Function to slice a given vector
// from range x to y
// https://www.techiedelight.com/get-slice-sub-vector-from-vector-cpp/
std::vector<Token> slicing(std::vector<Token>& v, int x, int y) {
  auto first = v.cbegin() + x;
  auto last = v.cbegin() + y + 1;

  std::vector<Token> vec(first, last);
  return vec;
}

Parser::Parser(std::string passed_data) {
  data = passed_data;
  lexer = Lexer(data);
  tokens = lexer.getTokens();
  program = Parse(tokens);
}

std::vector<Node*> Parser::Parse(std::vector<Token> to_parse) {
  std::vector<Node*> output;
  for (int i = 0; i < to_parse.size(); i++) {
    auto token = to_parse[i];
    if (token.tokenString == "include" || token.tokenString == "return") {
      // include and return have the same implementation for now
      auto response = Include(to_parse, token, i);
      i = std::get<1>(response);
      output.push_back(std::get<0>(response));
    } else if (token.tokenString == "fun") {
      auto response = Fun(to_parse, token, i);
      i = std::get<1>(response);
      output.push_back(std::get<0>(response));
    } else if (token.tokenString == "(") {
      auto response = FunCall(to_parse, token, i);
      i = std::get<1>(response);
    }
  }

  return output;
}

std::tuple<Node*, int> Parser::Include(std::vector<Token> to_parse, Token token,
                                       int i) {
  auto n = new Node();
  n->type = NodeTypes::T_STATEMENT;
  n->data->s.action = token.tokenString;
  n->main = token.tokenString;
  n->data->mode = ND_Mode::ST;
  auto response = lexer.getTillEOLOrBlock(to_parse, i, false);
  n->data->s.parameters = std::get<0>(response);

  return std::tuple<Node*, int>{n, std::get<1>(response)};
}

std::tuple<Node*, int> Parser::Fun(std::vector<Token> to_parse, Token token,
                                   int i) {
  auto response = lexer.getTillEOLOrBlock(to_parse, i, true);
  auto data = std::get<0>(response);
  // get the entire identifier
  auto iter = i;
  std::vector<Token> identifier;
  auto currentChar = to_parse[iter];
  while (currentChar.type != TokenTypes::SYMBOL) {
    identifier.push_back(currentChar);
    currentChar = to_parse[++iter];
  }

  auto rawblockBegin =
      find(data.begin(), data.end(), Token(TokenTypes::BLOCK, "{", 0, 0));
  if (rawblockBegin == data.end()) {
    std::cout << "Missing the function block\n";
    std::exit(1);
  }
  auto blockBegin = rawblockBegin - data.begin();

  auto blockEnd =
      find(data.begin(), data.end(), Token(TokenTypes::BLOCK_END, "}", 0, 0)) -
      data.begin();
  // check is not needed because line 57 lexer.cpp

  // getting the function block
  // indexs are +1 & -1 to remove EOLs
  // ! don't make it +2 and -2, identifier fetcher in FunCall breaks
  auto block = slicing(data, blockBegin + 1, blockEnd - 1);

  // get the token before "{" -> which is the return type
  auto returnTypeIndex = blockBegin - 1;
  auto returnType = data[returnTypeIndex];

  auto raw_arguments = slicing(data, 2, returnTypeIndex - 1);
  std::vector<Argument> arguments;
  for (int i = 0; i < raw_arguments.size(); i++) {
    auto arg = raw_arguments[i];

    // Funfact:
    // fun x(y -> int str) {}
    // y's type will be str

    if (arg.type == TokenTypes::TYPE)
      arguments[arguments.size() - 1].type = typeFromstr(arg.tokenString);
    if (arg.type == TokenTypes::IDENTIFIER) {
      Argument* a = new Argument();
      a->identifier = arg;

      arguments.push_back(*a);
    }
  }

  auto n = new Node();
  n->type = NodeTypes::T_FUNCTION;
  n->main = token.tokenString;
  n->data = new NodeData();
  n->data->f = Function();
  n->data->mode = ND_Mode::FUN;
  n->data->f.arguments = arguments;
  n->data->f.nodes = Parse(block);
  // n->data->f.identifier = identifier;  // !
  n->data->f.returnType = returnType;

  return std::tuple<Node*, int>{n, std::get<1>(response)};
}

std::tuple<Node*, int> Parser::FunCall(std::vector<Token> to_parse, Token token,
                                       int i) {
  // find the entire identifier
  auto iter = i;
  std::vector<Token> identifier;
  auto currentChar = to_parse[iter];
  while (currentChar.type != TokenTypes::EOL) {
    identifier.push_back(currentChar);
    currentChar = to_parse[--iter];
  }
  // flip it to correct order
  std::reverse(identifier.begin(), identifier.end());
  iter = i;
  currentChar = to_parse[iter];
  while (currentChar.tokenString != ")") currentChar = to_parse[++iter];
  auto IndexOfArgumentEnd = iter;
  auto raw_arguments = slicing(to_parse, i, IndexOfArgumentEnd);
  std::vector<Argument_N_Literal*> arguments;
  bool gettingString = false;
  for (int i = 0; i < raw_arguments.size(); i++) {
    auto currentArgument = raw_arguments[i];
    auto anl = new Argument_N_Literal();
    if (vecFromStr(currentArgument.tokenString)[0] == '\"') {  // string
      anl->literal.literal.tokenString = currentArgument.tokenString;
      anl->literal.literal.line = currentArgument.line;
      anl->literal.literal.column = currentArgument.column;
      anl->literal.type = Types::str;
    } else if (IsDigit(vecFromStr(currentArgument.tokenString)[0])) {  // number
      anl->literal.literal.tokenString = currentArgument.tokenString;
      anl->literal.literal.line = currentArgument.line;
      anl->literal.literal.column = currentArgument.column;
      anl->literal.type = Types::i32;
    } else {
      anl->argument.identifier.tokenString = currentArgument.tokenString;
      anl->argument.identifier.line = currentArgument.line;
      anl->argument.identifier.column = currentArgument.column;
      anl->argument.type = Types::str;
      // TODO: look at the AST and find the identifier, and put type of the
      // identifier here
    }
  }

  auto func = new Function_Call();
  func->arguments = arguments;
  func->identifer = identifier;

  std::string main = "";
  for (auto arg : arguments)
    if (arg->mode == ANL_Mode::Arg)
      main += arg->argument.identifier.tokenString;
    else
      main += arg->literal.literal.tokenString;

  auto n = new Node();
  n->data->mode = ND_Mode::FUNC;
  n->data->fc = func;
  n->main = main;
  n->type = NodeTypes::T_FUNCTION_CALL;

  return std::tuple<Node*, int>{n, i};
}