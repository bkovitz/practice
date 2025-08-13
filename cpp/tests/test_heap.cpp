#include <gtest/gtest.h>
#include "util.h"
#include "heap.h"

using namespace heap;

TEST(TestHeap, IsMinHeap) {
  EXPECT_TRUE(is_min_heap({1, 3, 7, 9, 10, 8, 14, 15, 13, 16}));
  EXPECT_FALSE(is_min_heap({1, 9, 7, 3, 10, 8, 14, 15, 13, 16}));
}

TEST(TestHeap, Heapify) {
  //std::vector<int> vec{1, 16, 7, 3, 10, 14, 8, 15, 13, 9};
  std::vector<int> vec{1, 9, 7, 3, 10, 8, 14, 15, 13, 16};
  heapify(vec, 1);
  EXPECT_TRUE(is_min_heap(vec));
}

TEST(TestHeap, BuildMinHeap) {
  std::vector<int> vec{1, 16, 7, 3, 10, 14, 8, 15, 13, 9};
  //std::cout << "HERE1" << std::endl;
  build_min_heap(vec);
  //std::cout << "HERE2" << std::endl;
  //std::cout << vec << std::endl;
  EXPECT_TRUE(is_min_heap(vec));
}

TEST(TestHeap, HeapSort) {
  std::vector<int> vec{1, 16, 7, 3, 10, 14, 8, 15, 13, 9};
  //std::cout << "HERE1" << std::endl;
  heapsort(vec);
  //std::cout << "HERE2" << std::endl;
  //std::cout << vec << std::endl;
  std::vector<int> expect{1, 3, 7, 8, 9, 10, 13, 14, 15, 16};
  EXPECT_EQ(vec, expect);
}
