#include <iostream>

int getNameLength() {
  std::cout << "Please enter your full name: ";

  std::string full_name{};
  std::getline(std::cin >> std::ws, full_name);
  return static_cast<int>(full_name.length());
}

int getAge() {
  std::cout << "Please enter your age: ";

  int age{};
  std::cin >> age;
  return age;
}
int main() {
  int nameLength(getNameLength());
  int age(getAge());
  std::cout << "Your age + length of name is: " << nameLength + age << "\n";
  return 0;
}
