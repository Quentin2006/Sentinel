// broken_syntax.cpp - Missing semicolons and syntax errors
// Use: sentinel g++ test/broken_syntax.cpp -o test/broken_syntax
// Expected: Sentinel should detect and fix the missing semicolons

#include <iostream>

int main() {
    int x = 5;  // ERROR: missing semicolon
    int y = 10; 
    
    std::cout << "x + y = " << x + y << std::endl;  // ERROR: missing semicolon
    
    return 0;
}
