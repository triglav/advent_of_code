#include <iostream>

int CalculateFuel(int mass) {
  return mass / 3 - 2;
}

int CalculateFuel_correct(int mass) {
  auto fuel = CalculateFuel(mass);
  if (fuel <= 0) {
    return 0;
  }
  return fuel + CalculateFuel_correct(fuel);
}

int main() {
  int total_fuel = 0;
  int total_fuel_correct = 0;
  int module_mass;
  while (std::cin >> module_mass) {
    total_fuel += CalculateFuel(module_mass);
    total_fuel_correct += CalculateFuel_correct(module_mass);
  }
  std::cout
    << total_fuel << "\n"
    << total_fuel_correct << "\n";
  return 0;
}

