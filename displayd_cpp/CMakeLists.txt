cmake_minimum_required(VERSION 3.16)
project(displayd LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(displayd
  src/main.cpp
)

target_include_directories(displayd PRIVATE include)

if (CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
  target_compile_options(displayd PRIVATE -Wall -Wextra -Wpedantic)
endif()