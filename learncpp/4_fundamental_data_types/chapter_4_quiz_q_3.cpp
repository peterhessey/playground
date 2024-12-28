#include <iostream>

double getTowerHeight() {
  std::cout << "Please enter the height of the tower in metres: ";

  double towerHeight{};
  std::cin >> towerHeight;
  return towerHeight;
}

double calculateBallHeightAtTime(double towerHeight, double time) {
  double gravity{9.8};
  double distanceFallen{gravity * time * time / 2};
  double finalHeight{towerHeight - distanceFallen};

  if (finalHeight < 0)
    return 0.0;
  return finalHeight;
}

void printBallHeightAtTime(double ballHeight, int timeInSeconds) {
  if (ballHeight > 0.0)
    std::cout << "At " << timeInSeconds << " seconds, the ball is at height "
              << ballHeight << " metres" << "\n";
  else
    std::cout << "At " << timeInSeconds
              << " seconds, the ball is on the ground.\n";
}

int main() {
  double towerHeight{getTowerHeight()};

  for (int i = 0; i <= 5; i++) {
    double heightAtTime{calculateBallHeightAtTime(towerHeight, i)};
    printBallHeightAtTime(heightAtTime, i);
  }
  return 0;
}
