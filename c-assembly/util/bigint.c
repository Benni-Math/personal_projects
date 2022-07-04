/**
*   Simple implementation of BigInt (via buffer)
*/

#include <bigint.h>

#define B_SIZE 3000

struct buffer {
    size_t index;
    char data[B_SIZE];
};

void init_buffer(struct buffer *buffer, int n) {
    for (buffer->index = B_SIZE; 
        n; buffer->data[--buffer->index] = (char) (n%10), n/= 10);
}


