cmake_minimum_required(VERSION 3.14)

project(practice)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(extern/googletest)
enable_testing()
add_subdirectory(tests)






