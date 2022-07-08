#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

int main(int arc, char **argv) {
    FILE *file = fopen("/etc/hosts", "r");
    if (!file) {
        fprintf(stderr, "could not open file\n");
        return 1;
    }

    const size_t buffer_size = 16;
    char *buffer = malloc(buffer_size);

    while (true) {
        size_t size = fread(buffer, 1, buffer_size - 1, file);
        if (size == 0) {
            break;
        }

        buffer[size] = '\0';
        printf("%s", buffer);
    }

    printf("\n");
    return 0;
}

