#include <iostream>
#include <string>
#include <string_view>

std::string getPersonName(int personNumber) {
  std::cout << "Enter name of user #" << personNumber << ": ";

  std::string personName{};
  std::getline(std::cin >> std::ws, personName);
  return personName;
}

int getPersonAge(std::string_view personName) {
  std::cout << "Enter the age of " << personName << ": ";

  int personAge{};
  std::cin >> personAge;
  return personAge;
}

int main() {
  std::string name1 = getPersonName(1);
  std::string name2 = getPersonName(2);

  int age1{getPersonAge(name1)};
  int age2{getPersonAge(name2)};

  if (age1 < age2) {
    std::cout << name1 << " (age " << age1 << ") is younger than " << name2
              << " (age " << age2 << ").\n";
    return 0;
  }

  std::cout << name2 << " (age " << age2 << ") is younger than " << name1
            << " (age " << age1 << ").\n";
  return 0;
}
