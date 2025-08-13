#include <iostream>

template<typename T>
std::ostream& operator<<(std::ostream& os, const std::vector<T>& vec) {
  bool first_time = false;
  os << '{';
  for (const T& t : vec) {
    if (!first_time)
      os << ' ';
    os << t;
    first_time = false;
  }
  os << '}';
  return os;
}
