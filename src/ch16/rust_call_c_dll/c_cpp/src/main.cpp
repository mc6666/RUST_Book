#include <iostream>

#include "lib.h"

int main() {
    const char * s = introduce("Srikanth", 31);
    std::cout << s << std::endl;

    deallocate_string(s);

    return 0;
}
