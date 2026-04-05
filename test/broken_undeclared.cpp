// broken_undeclared.cpp - Undeclared identifier errors
// Use: sentinel g++ test/broken_undeclared.cpp -o test/broken_undeclared
// Expected: Sentinel should detect undeclared variables/typos

#include <iostream>

int main() {
    int counter = 0;
    
    couter++;  // ERROR: typo - should be 'counter'
    
    std::cout << "Counter: " << counter << std::endl;
    
    result = 42;  // ERROR: 'result' was never declared
    
    return 0;
}
