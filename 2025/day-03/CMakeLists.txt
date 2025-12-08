cmake_minimum_required(VERSION 3.10)
project(day-03)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(bank STATIC src/bank.cpp)
target_include_directories(bank PUBLIC src)

add_executable(day-03 src/main.cpp)
target_link_libraries(day-03 PRIVATE bank)

# Enable testing
enable_testing()

# Add test executable (assuming tests are in tests/ directory)
add_executable(day-03-tests tests/test_bank.cpp)
target_link_libraries(day-03-tests PRIVATE bank)

# Add test
add_test(NAME day-03-tests COMMAND day-03-tests)