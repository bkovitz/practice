#include <iostream>
#include <algorithm>
#include "heap.h"

namespace heap {

  using V = std::vector<int>;

  inline
  index_t
  left(const V& vec, index_t i) {
    return (i << 1) + 1;
  }

  inline
  index_t
  right(const V& vec, index_t i) {
    return (i << 1) + 2;
  }

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

  void
  heapify(std::vector<int>& vec, index_t i) {
    for ( ; ; ) {
      index_t l = left(vec, i);
      index_t r = right(vec, i);
      index_t i_least;
      if (l < vec.size() && vec[l] < vec[i])
        i_least = l;
      else
        i_least = i;
      if (r < vec.size() && vec[r] < vec[i])
        i_least = r;
      if (i_least != i) {
        std::swap(vec[i], vec[i_least]);
        continue;
      }
      break;
    }
  }
}
