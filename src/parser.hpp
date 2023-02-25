#pragma once

#include "lexer.hpp"
#include "util.hpp"

/*
  Function Argument

  eg.
  _argc -> i32
*/
struct Argument {
  Token identifier;
  Types type;
  // https://stackoverflow.com/questions/321351/initializing-a-union-with-a-non-trivial-constructor
  Argument();
};

/*
  Literals

  eg. 0, "Hello World"
*/
struct Literal {
  Token literal;
  Types type;
};

// used to figure out which one has actual data
enum ANL_Mode { Arg, Lit };
/*
  Argument + Literal
*/
union Argument_N_Literal {
  ANL_Mode mode;
  Argument argument;
  Literal literal;
  // https://stackoverflow.com/questions/321351/initializing-a-union-with-a-non-trivial-constructor
  Argument_N_Literal();
};

/*
  Function

  eg.
  fun main(_argc -> i32, _argv -> str[]) => u8 {...}
*/
struct Function {
  std::vector<Token> identifier;
  Token returnType;
  std::vector<Argument> arguments;
  std::vector<Node*> nodes;
};

/*
  Function call

  eg.
  io->out("Hello World")
*/
struct Function_Call {
  std::vector<Token> identifer;
  std::vector<Argument_N_Literal*> arguments;
};

// used to figure out which one has actual data
enum ND_Mode { ST, FUN, FUNC };

union NodeData {
  ND_Mode mode;
  Statement s;
  Function f;
  Function_Call* fc;

  // https://stackoverflow.com/questions/321351/initializing-a-union-with-a-non-trivial-constructor
  NodeData();
};

/*
  Data cell in the AST
*/
class Node {
 public:
  NodeTypes type;
  // used for printing purposes
  std::string main;
  NodeData* data = new NodeData();
  Node();
};

/*
  AST Generator
*/
class Parser {
 public:
  std::string data;
  std::vector<Token> tokens;
  Lexer lexer = Lexer(data);
  std::vector<Node*> program;
  Parser(std::string passed_data);
  std::vector<Node*> Parse(std::vector<Token> data);
  /*
    Returns:
      Node -> Node to add
      position -> position to skip to after the function
  */
  std::tuple<Node*, int> Include(std::vector<Token> to_parse, Token token,
                                 int i);
  /*
  Returns:
    Node -> Node to add
    position -> position to skip to after the function
  */
  std::tuple<Node*, int> Fun(std::vector<Token> to_parse, Token token, int i);
  /*
  Returns:
    Node -> Node to add
    position -> position to skip to after the function
  */
  std::tuple<Node*, int> FunCall(std::vector<Token> to_parse, Token token,
                                 int i);
};