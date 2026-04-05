// broken_braces.cpp - Unbalanced braces
// Use: sentinel g++ test/broken_braces.cpp -o test/broken_braces
// Expected: Sentinel should detect missing closing brace

#include <iostream>

int calculate(int a, int b) {
  if (a > b) {
    return a - b;
  }
  return a + b;
}

int main() {
    std::cout << calculate(10, 5) << std::endl;
    return 0;
  }
