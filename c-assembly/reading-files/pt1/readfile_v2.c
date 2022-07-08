#include <stdlib.h>
#include <stdbool.h>
#include <stdio.h>

#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

int main(int argc, char **argv) {
    int fd = open("/etc/hosts", O_RDONLY);
    // oh look, this time '0' does not mean error!
    if (fd == -1) {
        fprintf(stderr, "could not open file\n");
        return 1;
    }

    const size_t buffer_size = 16;
    char *buffer = malloc(buffer_size);

    while (true) {
        // this time the size is signed!
        ssize_t size = read(fd, buffer, buffer_size);
        
        if (size == -1) {
            fprintf(stderr, "an error happened while reading the file");
            return 1;
        }
        
        if (size == 0) {
            break;
        }

        write(STDOUT_FILENO, buffer, size);
    }

    write(STDOUT_FILENO, "\n", 1);
    return 0;
}

