// valid_math.cpp - A working C++ program with basic math operations
// Use: sentinel g++ test/valid_math.cpp -o test/valid_math
// Expected: Compiles without errors

#include <iostream>

int add(int a, int b) {
    return a + b;
}

int multiply(int a, int b) {
    return a * b;
}

int main() {
    int x = 5;
    int y = 10;
    
    std::cout << "Add: " << add(x, y) << std::endl;
    std::cout << "Multiply: " << multiply(x, y) << std::endl;
    
    return 0;
}
