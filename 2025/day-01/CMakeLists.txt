cmake_minimum_required(VERSION 3.10)
project(day-01)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(safe STATIC src/safe.cpp)
target_include_directories(safe PUBLIC src)

add_executable(day-01 src/main.cpp)
target_link_libraries(day-01 PRIVATE safe)

# Enable testing
enable_testing()

# Add test executable (assuming tests are in tests/ directory)
add_executable(day-01-tests tests/test_safe.cpp)
target_link_libraries(day-01-tests PRIVATE safe)

# Add test
add_test(NAME day-01-tests COMMAND day-01-tests)