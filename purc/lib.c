#include <stdio.h>

#include "lib.h"

uint32_t PURC_API purc_max(uint32_t x, uint32_t y) {
    printf("x = %d, y = %d\n", x, y);
    return x > y ? x : y;
}

void PURC_API purc_call_rs(void (*cb)()) {
    if(NULL == cb) {
        printf("cb is null");
    } else {
        cb();
    }
}
