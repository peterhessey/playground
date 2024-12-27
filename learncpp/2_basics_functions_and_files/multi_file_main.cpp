#include "add.h"
#include <iostream>

int main() {
  std::cout << "Please enter two integers: ";
  int x{};
  std::cin >> x;

  int y{};
  std::cin >> y;

  std::cout << "Adding these across two files gives: " << add(x, y) << "\n";
}
