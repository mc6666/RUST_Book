#include "pch.h"
#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>


extern  "C" {
    void rust_function(); 
    int32_t add_numbers(int32_t a, int32_t b);
}

int main() {
    int32_t x = add_numbers(25, 17);
    printf("x=%d\n", x);
    rust_function();
    return 0;
}