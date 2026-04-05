// broken_type.cpp - Type mismatch errors
// Use: sentinel g++ test/broken_type.cpp -o test/broken_type
// Expected: Sentinel should detect type mismatches

#include <iostream>
#include <string>

int main() {
    int number = "hello";  // ERROR: cannot assign string literal to int
    
    std::string text = 42;  // ERROR: cannot assign int to string
    
    double value = 3.14;
    int truncated = value;  // WARNING: implicit conversion (this is valid but may warn)
    
    std::cout << truncated << std::endl;
    
    return 0;
}
