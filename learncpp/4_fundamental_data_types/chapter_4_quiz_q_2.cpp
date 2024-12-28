#include <iostream>

double getDoubleInput() {
  std::cout << "Please enter a double value: ";
  double answer{};
  std::cin >> answer;
  return answer;
}

char getOperationInput() {
  std::cout << "Please enter an operator (+, -, / or *): ";
  char answer{};
  std::cin >> answer;
  return answer;
}

double performOperation(double value_1, double value_2, char operation) {
  if (operation == '+')
    return value_1 + value_2;
  else if (operation == '-')
    return value_1 - value_2;
  else if (operation == '/')
    return value_1 / value_2;
  else if (operation == '*')
    return value_1 * value_2;

  std::cout << "Invalid operation char: " << operation << ", returning -69"
            << "\n";
  return -69;
}

int main() {
  double value_1{getDoubleInput()};
  double value_2{getDoubleInput()};
  char operation{getOperationInput()};
  double answer{performOperation(value_1, value_2, operation)};
  std::cout << value_1 << " " << operation << " " << value_2 << " = " << answer
            << "\n";
}
