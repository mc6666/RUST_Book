#include <sstream>
#include <cstdlib>
#include <cstring>
#include "lib.h"

std::stringstream ss;

const char * introduce(const char * name, int age) {
    ss.clear();
    ss << "Hi, I am " << name << ". My age is " << age << ".";
    return strdup(ss.str().c_str());
}

void deallocate_string(const char * s) {
    free((void*)s);
}
