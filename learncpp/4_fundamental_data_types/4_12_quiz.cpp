#include <iostream>

int main() {
  // how to do it with `static_cast`
  std::cout << "Enter a single character: ";

  char input_character{};
  std::cin >> input_character;

  std::cout << "The ASCII repr of your input char is: "
            << static_cast<int>(input_character) << "\n";

  // how to do it implicitly (can also use fns)
  std::cout << "Enter a single character: ";

  char input_character_2{};
  std::cin >> input_character_2;

  int input_char_as_int{input_character_2};

  std::cout << "Here's an implicit type conversion example "
            << input_char_as_int << "\n";
}
