#include <array>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>

#include <openssl/md5.h>

std::string md5(std::string const &secret, int number) {
  std::array<unsigned char, MD5_DIGEST_LENGTH> digest{0};
  auto token = secret + std::to_string(number);
  MD5(reinterpret_cast<unsigned char const *>(token.data()), token.size(),
      digest.data());

  std::stringstream ss;
  for (auto x : digest) {
    ss << std::hex << std::setw(2) << std::setfill('0') << static_cast<int>(x);
  }
  return ss.str();
}

int main() {
  std::string secret = "ckczppom";

  for (int i = 100000; i < 999999; ++i) {
    auto hash = md5(secret, i);
    if (hash.rfind("00000", 0) == 0) {
      std::cout << i << "\n";
      break;
    }
  }
  return 0;
}
