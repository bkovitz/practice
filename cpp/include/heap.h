#include <vector>
#include <cstddef>

namespace heap {
  using index_t = std::ptrdiff_t;
  bool is_min_heap(const std::vector<int>& vec);
  void heapify(std::vector<int>& vec, index_t i);
  void build_min_heap(std::vector<int>& vec);
  void heapsort(std::vector<int>& vec);
}
