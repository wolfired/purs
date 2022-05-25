#include <sys/stat.h>
#include <sys/mman.h>

#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>

#include "lib.h"

int main(int argc, char** argv) {
    return EXIT_SUCCESS;
}

void print_syscall_info() {
    printf("size of stat struct: %zu\n", sizeof(struct stat));
    printf("offset of st_size  : %zu\n", offsetof(struct stat, st_size));
    printf("PROT_READ   = 0x%x\n", PROT_READ);
    printf("MAP_PRIVATE = 0x%x\n", MAP_PRIVATE);
}
