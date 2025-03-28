cmake_minimum_required(VERSION 3.0)
project(LibSandbox C)

add_library(sandbox SHARED main.c)

install(TARGETS sandbox DESTINATION .)