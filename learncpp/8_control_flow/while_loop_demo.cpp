#include <iostream>
int main() {
  char letter{'a'};
  while (letter <= 'z') {
    std::cout << letter << " ";
    letter += 1;
  }
  std::cout << "End turn\n\n";

  const int n_cols{5};
  int outer{n_cols};

  while (outer >= 1) {
    int inner{outer};
    while (inner > 0) {
      std::cout << inner << " ";
      inner -= 1;
    }
    outer -= 1;
    std::cout << "\n";
  }

  std::cout << "\n\n";

  int outer_counter = 1;

  while (outer_counter <= n_cols) {
    int diff{n_cols - outer_counter};

    if (diff > 0) {
      int diff_printed_count{0};
      while (diff_printed_count < diff) {
        std::cout << "  ";
        ++diff_printed_count;
      }
    }

    int inner_counter{n_cols - diff};
    while (inner_counter > 0) {
      std::cout << inner_counter << " ";
      --inner_counter;
    }
    std::cout << "\n";
    ++outer_counter;
  }
}
