#include <unordered_set>
#include <string>
#include <cctype>
#include "docsim.h"

std::unordered_set<std::string>
make_word_set(std::string s) {
  std::unordered_set<std::string> result;
  std::string word;
  enum { BETWEEN_WORDS, IN_WORD } state = BETWEEN_WORDS;

  for (auto c : s) {
    switch (state) {
    case BETWEEN_WORDS:
      if (std::isalpha(c)) {
        word = std::tolower(c);
        state = IN_WORD;
      } else {
        /* nothing */
      }
      break;
    case IN_WORD:
      if (std::isalpha(c)) {
        word.push_back(std::tolower(c));
      } else {
        if (!word.empty())
          result.insert(word);
        state = BETWEEN_WORDS;
      }
      break;
    }
  }
  if (state == BETWEEN_WORDS)
    result.insert(word);

  return result;
}

double similarity(std::string q, std::string d) {
  auto qset = make_word_set(q);
  auto dset = make_word_set(d);

  return 0.0;
}

/* A substitute for main, to experiment with; not production code. */
void run() {
  
}
