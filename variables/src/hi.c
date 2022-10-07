#include <stdint.h>
#include <stdio.h>

int main() {
    uint8_t bytes[] = { 72, 105, 0 };
    puts((char*)bytes);
    return 0;
}
