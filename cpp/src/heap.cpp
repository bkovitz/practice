#include <iostream>
#include "heap.h"

namespace heap {

  using index_t = std::ptrdiff_t;
  using V = std::vector<int>;

  inline
  index_t 
  parent(const V& vec, index_t i) {
    return (i - 1) >> 1;
  }

  bool
  is_min_heap(const std::vector<int>& vec) {
    for (index_t i = 1; i < vec.size(); i++) {
      index_t p = parent(vec, i);
      std::cout << p << ' ' << i << ' ' << vec[p] << ' ' << vec[i] << std::endl;
      if (!(vec[p] <= vec[i]))
        return false;
    }
    return true;
  }

}
