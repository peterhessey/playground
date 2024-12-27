#include <iostream>

int getIntegerFromUser() {
  std::cout << "Please enter an integer: ";
  int number{};
  std::cin >> number;

  return number;
}
int doubleNumber(int number) { return number * 2; }

int main() {
  int userNumber{getIntegerFromUser()};
  std::cout << "That doubled with functions is: " << doubleNumber(userNumber)
            << "\n";

  return 0;
}
