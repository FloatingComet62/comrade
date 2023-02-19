#include <vector>
#include <iostream>
#include <functional>

template <typename T>
void print_vec(std::vector<T> vec, std::function<std::string(T)> stringify) {
  std::cout << "[";
  for (int i = 0; i < vec.size(); i++) {
    bool isLast = i+1 == vec.size();
    std::cout << stringify(vec.at(i)) << (isLast ? "" : ", ");
  }
  std::cout << "]\n";
}