#include <iostream>

int CalculateFuel(int mass) {
  return mass / 3 - 2;
}

int main() {
  int total_fuel = 0;
  int module_mass;
  while (std::cin >> module_mass) {
    total_fuel += CalculateFuel(module_mass);
  }
  std::cout << total_fuel << "\n";
  return 0;
}

