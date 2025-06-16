#include <gtest/gtest.h>
#include "add.h"

TEST(TestAdd, AddsCorrectly) {
  EXPECT_EQ(add(2, 3), 5);
}
