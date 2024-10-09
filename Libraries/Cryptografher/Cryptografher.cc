#include "Cryptografher.hh"

int main() {
  std::string key = generateNewKey(12);
  std::cout << key << std::endl;
}
