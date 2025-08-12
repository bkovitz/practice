add_executable(test_add test_add.cpp ../src/add.cpp)
target_include_directories(test_add PRIVATE ../include)
target_link_libraries(test_add gtest_main)

add_executable(test_docsim test_docsim.cpp ../src/docsim.cpp)
target_include_directories(test_docsim PRIVATE ../include)
target_link_libraries(test_docsim gtest_main)
#add_test(NAME TestDocsim COMMAND test_docsim)

add_executable(test_heap test_heap.cpp ../src/heap.cpp)
target_include_directories(test_heap PRIVATE ../include)
target_link_libraries(test_heap gtest_main)

include(GoogleTest)
gtest_discover_tests(test_add)
gtest_discover_tests(test_docsim)
gtest_discover_tests(test_heap)
