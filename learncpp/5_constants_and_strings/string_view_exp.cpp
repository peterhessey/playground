#include <iostream>
#include <string>
#include <string_view>

std::string_view getUndefinedBoolName(bool b) {
  std::string t{"true"};  // local variable
  std::string f{"false"}; // local variable

  if (b)
    return t; // return a std::string_view viewing t

  return f; // return a std::string_view viewing f
} // t and f are destroyed at the end of the function

std::string_view getDefinedBoolName(bool b) {
  // returning C-style string literals as views is okay!
  if (b)
    return "true";

  return "false";
}

int main() {
  std::cout << "Undefined: " << getUndefinedBoolName(true) << " "
            << getUndefinedBoolName(false) << "\n"; // undefined behavior

  std::cout << "Defined: " << getDefinedBoolName(false) << " "
            << getDefinedBoolName(true) << "\n"; // undefined behavior

  return 0;
}
