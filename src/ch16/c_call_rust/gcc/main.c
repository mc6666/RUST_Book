#include <stdio.h>

extern int rust_function();

int main() {
    int result = rust_function();
    printf("Called Rust function, result: %d\n", result);
    return 0;
}
