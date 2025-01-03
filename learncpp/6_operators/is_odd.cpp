#include <iostream>

int getInteger() {
  std::cout << "Enter an integer: ";

  int userValue{};
  std::cin >> userValue;
  return userValue;
}

constexpr bool isOdd(int userValue) { return (userValue % 2) != 0; }

int main() {
  int userValue{getInteger()};

  if (isOdd(userValue))
    std::cout << userValue << " is odd\n";
  else
    std::cout << userValue << " is even\n";

  return 0;
}
