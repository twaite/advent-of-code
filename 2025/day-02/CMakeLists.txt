cmake_minimum_required(VERSION 3.10)
project(day-02)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(product STATIC src/product.cpp)
target_include_directories(product PUBLIC src)

add_executable(day-02 src/main.cpp)
target_link_libraries(day-02 PRIVATE product)

# Enable testing
enable_testing()

# Add test executable (assuming tests are in tests/ directory)
add_executable(day-02-tests tests/test_product.cpp)
target_link_libraries(day-02-tests PRIVATE product)

# Add test
add_test(NAME day-02-tests COMMAND day-02-tests)