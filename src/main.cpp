#include <fstream>
#include <iostream>
#include <string>

#include "parser.hpp"

std::string ReadFile(std::string path) {
  std::string out = "";
  std::string line;
  std::ifstream file = std::ifstream(path);
  while (getline(file, line)) out += line + "\n";
  return out;
}

int main() {
  // std::string l = ReadFile("../exmaples/hello_world.txt");
  std::string data =
      "include std->io\n\nfun main(_argc -> i32, _argv -> str[]) => u8 "
      "{\n\tio->out(\"Hello World\")\n\treturn 0\n}";
  Parser parser = Parser(data);
  return 0;
}