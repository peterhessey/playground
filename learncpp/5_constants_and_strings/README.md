# Chapter 5 - Constants and Strings

## 5.7 - Introduction to `std::string`

- This is a modern alternative to "C-style" strings (the default type for double-quoted stuff)
- Use `std::getline()` to retrieve input text that includes whitespace, e.g. `std::getline(std::cin >> std::ws, param_to_store_value)`
  - The use of `std::ws` here strips whitespace off of the start of the string (e.g. any leftover "`\n`" that may still be in the buffer)
- Use `std::string.length()` member function to get the length of strings (returns an unsigned integer!!!)
- Passing `std::string` objects by value to other functions is expensive as the string is copied.
  - In `C++17` the `std::string_view` object is added to address this (can pointers also be used?)
  - This "problem" / inefficiency with `std::string` does not apply to returning them from functions (in most cases)
