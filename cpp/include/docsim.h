#pragma once

#include <unordered_set>
#include <string>

void run();
double similarity(std::string, std::string);
std::unordered_set<std::string> make_word_set(std::string);
