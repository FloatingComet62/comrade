#include <fstream>
#include <iostream>
#include <string>
#include "lexer.h"

std::string ReadFile(std::string Path){
	std::ifstream file(Path);
	file.seekg(0, std::ios::end);
	size_t size = file.tellg();
	std::string buffer(size, ' ');
	file.seekg(0);
	file.read(&buffer[0], size);
	return buffer;
}

int main() {
	std::cout << "Hello World";
	Lexer l = Lexer(ReadFile("hello_world.txt"));
	std::cout << "output: " << l.getToken(l.position);
	return 0;
}