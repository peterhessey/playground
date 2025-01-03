#include <iostream>
#include <string>

std::string getQuantityPhrase(int numApples) {
  if (numApples < 0)
    return "negative";
  else if (numApples == 0)
    return "no";
  else if (numApples == 1)
    return "a single";
  else if (numApples == 2)
    return "a couple of";
  else if (numApples == 3)
    return "a few";
  else
    return "many";
}

std::string getApplesPluralized(int numApples) {
  return (numApples == 1) ? "apple" : "apples";
}

int main() {
  constexpr int maryApples{3};
  std::cout << "Mary has " << getQuantityPhrase(maryApples) << ' '
            << getApplesPluralized(maryApples) << ".\n";

  std::cout << "How many apples do you have? ";
  int numApples{};
  std::cin >> numApples;

  std::cout << "You have " << getQuantityPhrase(numApples) << ' '
            << getApplesPluralized(numApples) << ".\n";

  return 0;
}
