// hello.cpp
#include <iostream>

extern "C" {
    const char* get_hello_world() {
        return "Hello Lucid";
    }
}