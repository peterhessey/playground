#include <iostream>

int main() {
  std::cout << "Select a number to count up to: ";

  int upperLimit{};
  std::cin >> upperLimit;

  for (int i{0}; i <= upperLimit; ++i) {

    bool wordFound = {false};
    if (i % 3 == 0) {
      std::cout << "fizz";
      wordFound = true;
    }
    if (i % 5 == 0) {
      std::cout << "buzz";
      wordFound = true;
    }
    if (i % 7 == 0) {
      std::cout << "pop";
      wordFound = true;
    }
    if (!wordFound) {
      std::cout << i;
    }
    std::cout << "\n";
  }
}
