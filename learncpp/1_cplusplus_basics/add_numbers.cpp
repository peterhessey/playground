#include <iostream>

int main() {

  std::cout << "Please enter an integer: ";

  int x{};
  std::cin >> x;

  std::cout << "Please enter another integer: ";

  int y{};
  std::cin >> y;

  std::cout << "The sum of those two integers is " << x + y << "\n";
}
