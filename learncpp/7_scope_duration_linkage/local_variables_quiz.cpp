#include <iostream>

int main() {
  std::cout << "Enter an integer: ";
  int smaller{};
  std::cin >> smaller;

  std::cout << "Enter a larger integer: ";
  int larger{};
  std::cin >> larger;

  if (smaller > larger) {
    std::cout << "Swapping, are you dumb?" << "\n";
    int tmp{smaller};
    smaller = larger;
    larger = tmp;
    // std::swap from <utility> can swap vars like this!
  } // tmp dies here

  // std::cout << "temp val: " << tmp << "\n";  // this would cause a compile
  // error

  std::cout << "Smallest val: " << smaller << "\n";
  std::cout << "Largest val: " << larger << "\n";
}
