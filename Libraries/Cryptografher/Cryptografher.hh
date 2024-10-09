#define CRYPTOGRAFHER_HH

#include <iostream>
#include <string>
#include "xxhash.h"  // Include the header for XXHash
#include <ctime>
#include <unistd.h>
 
// Now you can hash some strings via xxHash.
//    
// Usage:
//    std::string input = "Hello, World!";
//    size_t = hashStringXXH64(input, unsigned int seed);
//
size_t hashStringXXH64(const std::string &str, unsigned int seed) {
    return XXH64(str.data(), str.size(), seed);  // 0 is the seed
}

// Generate some random string as a key.
// 
// Usage example:
//    srand((unsigned)time(NULL) * getpid());
//    std::string key= generateNewKey(12);
std::string generateNewKey(const int len) {
    static const char alphanum[] =
        "0123456789"
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        "abcdefghijklmnopqrstuvwxyz";
    std::string tmp_s;
    tmp_s.reserve(len);

    for (int i = 0; i < len; ++i)
        tmp_s += alphanum[rand() % (sizeof(alphanum) - 1)];
    
    return tmp_s;
}


