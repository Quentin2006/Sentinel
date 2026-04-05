// broken_include.cpp - Missing or misspelled include
// Use: sentinel g++ test/broken_include.cpp -o test/broken_include
// Expected: Sentinel should detect missing include

#include <iostrem>  // ERROR: typo - should be <iostream>

int main() {
    std::cout << "This won't compile" << std::endl;
    return 0;
}
