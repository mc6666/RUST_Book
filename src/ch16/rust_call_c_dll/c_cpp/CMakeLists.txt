cmake_minimum_required(VERSION 3.22)

project("c_cpp")

include_directories(./include)

add_library(${PROJECT_NAME} SHARED ./src/lib.cpp)

add_executable(${PROJECT_NAME}_main ./src/main.cpp ./src/lib.cpp)

install(TARGETS ${PROJECT_NAME} DESTINATION .)
