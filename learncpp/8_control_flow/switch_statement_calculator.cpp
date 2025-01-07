#include <iostream>

int getInputInteger() {
  std::cout << "Select an integer: ";

  int input_integer{};
  std::cin >> input_integer;
  return input_integer;
}

char getInputOperator() {
  std::cout << "Select an operator: ";

  char input_operator{};
  std::cin >> input_operator;
  return input_operator;
}

void calculate(int integer_a, int integer_b, char chosen_operator) {
  int answer{};

  switch (chosen_operator) {
  case '+':
    answer = integer_a + integer_b;
    break;
  case '-':
    answer = integer_a - integer_b;
    break;
  case '*':
    answer = integer_a * integer_b;
    break;
  case '/':
    answer = integer_a / integer_b;
    break;
  default:
    std::cout << "Invalid operator: " << chosen_operator << "\n";
    return;
  }

  std::cout << integer_a << " " << chosen_operator << " " << integer_b << " ="
            << " " << answer << "\n";
}

int main() {
  int integer_a{getInputInteger()};
  int integer_b{getInputInteger()};
  char chosen_operator{getInputOperator()};

  calculate(integer_a, integer_b, chosen_operator);
}
