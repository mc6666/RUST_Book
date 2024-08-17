#include <stdio.h>
#include <stdint.h>
#include <windows.h>
#include <locale.h>

int32_t multiply(int32_t a, int32_t b) {
    // 必須修改控制台的地區設定
    setlocale(LC_ALL, "zh_TW.UTF-8"); 
    wprintf(L"呼叫成功\n");
    // SetConsoleOutputCP(65001);
    // printf("[C] 呼叫成功.\n");
    printf("[C] Hello from C!\n");
    printf("[C] Input a is: %i \n", a);
    printf("[C] Input b is: %i \n", b);
    printf("[C] Multiplying and returning result to Rust..\n");

    return a * b;
}
