add_executable(test_add test_add.cpp ../src/add.cpp)
target_include_directories(test_add PRIVATE ../include)
target_link_libraries(test_add gtest_main)

include(GoogleTest)
gtest_discover_tests(test_add)
