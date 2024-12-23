#include <iostream>
#include <ostream>

int main() {
  std::cout << "Hello" << " from here!" << std::endl;
  std::cout << "And here's something else for you 🐢" << std::endl;

  // use \n to add newlines without flushing buffer
  std::cout << "This line didn't have it's buffer flushed automatically :)"
            << "\n";

  // practicing stdin
  std::cout << "Enter your favourite number please: ";
  int x{};
  std::cin >> x;
  std::cout << "You picked: " << x << "\n";

  // doing the same but more
  std::cout << "please pick two integers separated by a space: ";
  int a{};
  int b{};

  std::cin >> a >> b;
  std::cout << "These are the two you picked: " << a << " and " << b << "\n";

  return 0;
}
