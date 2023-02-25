#pragma once

#include <cstring>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

// https://stackoverflow.com/questions/11413308/redefine-eof-as-character-string
#undef EOF

// https://stackoverflow.com/questions/5245774/how-to-use-class-which-defined-below
struct Token;
struct Node;

enum TokenTypes {
  KEYWORD,        // fun, match, include
  IDENTIFIER,     // x, y, z, i, j ,k
  TYPE,           // types
  SYMBOL,         // used for ( ) right now
  FUNCTION_CALL,  // used to figure out if a function is being called
  BLOCK,          // used for {
  BLOCK_END,      // used for }
  LITERAL,        // TODO
  EOL,            // End of Line
  EOF,            // End of File
};

enum Types {
  u4 = 1,
  u8 = 2,
  u16 = 3,
  u32 = 4,
  u64 = 5,
  u128 = 6,
  i4 = 7,
  i8 = 8,
  i16 = 9,
  i32 = 10,
  i64 = 11,
  i128 = 12,
  f4 = 13,
  f8 = 14,
  f16 = 15,
  f32 = 16,
  f64 = 17,
  f128 = 18,
  str = 19,
  b = 21,  // TODO: make it bool
  list = 32

  // Reason for choosing these numbers
  // 1-5 LSBs donate which type it is
  // and 6th bit represents if it is a list
  //
  // eg. 0001 0011 => 19 => str
  //     0011 0011 => 51 => str[]
};

/* Types of Nodes */
enum NodeTypes { T_STATEMENT, T_FUNCTION, T_FUNCTION_CALL };

/*
  Statements

  eg.
  include std->io
*/
struct Statement {
  std::string action;
  std::vector<Token> parameters;
};

// https://stackoverflow.com/questions/8263926/how-to-copy-stdstring-into-stdvectorchar
std::vector<char> vecFromStr(std::string x);

void crap(std::string message = "DEBUG");

Types typeFromstr(std::string string);