#include "util.hpp"

std::vector<char> vecFromStr(std::string x) {
  std::vector<char> output(x.begin(), x.end());
  return output;
}

void crap(std::string message) { std::cout << message << "\n"; }

Types typeFromstr(std::string string) {
  if (strcmp(string.c_str(), "u4")) return Types::u4;
  if (strcmp(string.c_str(), "u8")) return Types::u8;
  if (strcmp(string.c_str(), "u16")) return Types::u16;
  if (strcmp(string.c_str(), "u32")) return Types::u32;
  if (strcmp(string.c_str(), "u64")) return Types::u64;
  if (strcmp(string.c_str(), "u128")) return Types::u128;
  if (strcmp(string.c_str(), "i4")) return Types::i4;
  if (strcmp(string.c_str(), "i8")) return Types::i8;
  if (strcmp(string.c_str(), "i16")) return Types::i16;
  if (strcmp(string.c_str(), "i32")) return Types::i32;
  if (strcmp(string.c_str(), "i64")) return Types::i64;
  if (strcmp(string.c_str(), "i128")) return Types::i128;
  if (strcmp(string.c_str(), "f4")) return Types::f4;
  if (strcmp(string.c_str(), "f8")) return Types::f8;
  if (strcmp(string.c_str(), "f16")) return Types::f16;
  if (strcmp(string.c_str(), "f32")) return Types::f32;
  if (strcmp(string.c_str(), "f64")) return Types::f64;
  if (strcmp(string.c_str(), "f128")) return Types::f128;
  if (strcmp(string.c_str(), "str")) return Types::str;
  if (strcmp(string.c_str(), "bool")) return Types::b;
  if (strcmp(string.c_str(), "u4")) return Types::u4;
  if (strcmp(string.c_str(), "u8[]")) return (Types)(Types::u8 + Types::list);
  if (strcmp(string.c_str(), "u16[]")) return (Types)(Types::u16 + Types::list);
  if (strcmp(string.c_str(), "u32[]")) return (Types)(Types::u32 + Types::list);
  if (strcmp(string.c_str(), "u64[]")) return (Types)(Types::u64 + Types::list);
  if (strcmp(string.c_str(), "u128[]"))
    return (Types)(Types::u128 + Types::list);
  if (strcmp(string.c_str(), "i4[]")) return (Types)(Types::i4 + Types::list);
  if (strcmp(string.c_str(), "i8[]")) return (Types)(Types::i8 + Types::list);
  if (strcmp(string.c_str(), "i16[]")) return (Types)(Types::i16 + Types::list);
  if (strcmp(string.c_str(), "i32[]")) return (Types)(Types::i32 + Types::list);
  if (strcmp(string.c_str(), "i64[]")) return (Types)(Types::i64 + Types::list);
  if (strcmp(string.c_str(), "i128[]"))
    return (Types)(Types::i128 + Types::list);
  if (strcmp(string.c_str(), "f4[]")) return (Types)(Types::f4 + Types::list);
  if (strcmp(string.c_str(), "f8[]")) return (Types)(Types::f8 + Types::list);
  if (strcmp(string.c_str(), "f16[]")) return (Types)(Types::f16 + Types::list);
  if (strcmp(string.c_str(), "f32[]")) return (Types)(Types::f32 + Types::list);
  if (strcmp(string.c_str(), "f64[]")) return (Types)(Types::f64 + Types::list);
  if (strcmp(string.c_str(), "f128[]"))
    return (Types)(Types::f128 + Types::list);
  if (strcmp(string.c_str(), "str[]")) return (Types)(Types::str + Types::list);
  if (strcmp(string.c_str(), "bool[]")) return (Types)(Types::b + Types::list);
  return Types::str;
  // Probably throw a compiler error saying something like "Invalid type" or
  // something
}