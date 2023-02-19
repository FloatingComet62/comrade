#pragma once

#include <string>
#include <vector>
#include <cstring>

#include "lexer.hpp"

/* Types of Nodes */
enum NodeTypes {
  T_STATEMENT,
  T_FUNCTION,
  T_FUNCTION_CALL
};

/*
  Statements

  eg.
  include std->io
*/
struct Statement {
  std::string action;
  std::vector<Token> parameters;
};

/*
  Function Argument

  eg.
  _argc -> i32
*/
struct Argument {
  Token identifier;
  Token type;
  Argument() { memset(this, 0, sizeof(Argument)); }
};

/*
  Literals
  
  eg. 0, "Hello World"
*/
struct Literal {
  Token literal;
  Token type;
};

/*
  Argument + Literal
*/
union Argument_N_Literal {
  Argument argument;
  Literal literal;
  // https://stackoverflow.com/questions/321351/initializing-a-union-with-a-non-trivial-constructor
  Argument_N_Literal() { memset(this, 0, sizeof(Argument_N_Literal)); }
};

/*
  Function

  eg.
  fun main(_argc -> i32, _argv -> str[]) => u8 {...}
*/
struct Function {
  Token identifier; // TODO: std::vector<Token>
  Token returnType;
  std::vector<Argument> arguments;
};

/*
  Function call

  eg.
  io->out("Hello World")
*/
struct Function_Call {
  std::vector<Token> identifer;
  std::vector<Argument_N_Literal> arguments;
};

union NodeData {
  Statement s;
  Function f;
  Function_Call fc;

  // https://stackoverflow.com/questions/321351/initializing-a-union-with-a-non-trivial-constructor
  NodeData() { memset(this, 0, sizeof(NodeData)); }
};

/*
  Data cell in the AST
*/
class Node {
  public:
  NodeTypes type;
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
  Lexer lexer = Lexer(data);
  std::vector<Node*> program;

  Parser(std::string data);
};