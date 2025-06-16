#include <gtest/gtest.h>
#include "docsim.h"

TEST(TestDocsim, Run) {
  run();
}

TEST(TestDocsim, MakeWordSet) {
  const std::set<std::string> expect = {"blah", "de"};

  auto got = make_word_set(" Blah de blah.");
  EXPECT_EQ(got, expect);
}

TEST(TestDocsim, Basic) {
  std::string q = "que BLah?";
  std::string d = "Blah de BLAH, blah  blah ";

  EXPECT_FLOAT_EQ(similarity(q, d), 0.5);
}
