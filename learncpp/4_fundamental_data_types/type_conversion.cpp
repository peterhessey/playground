#include <iostream>

void print(int x) { std::cout << x << '\n'; }

int main() {
  double x{50.5};
  print(static_cast<int>(x));
#include <iostream>

  char ch{97}; // 97 is ASCII code for 'a'
  // print value of variable ch as an int
  std::cout << ch << " has value " << static_cast<int>(ch) << '\n';

  return 0;
}
