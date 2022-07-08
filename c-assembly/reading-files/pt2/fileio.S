// Various amcros to perfom file I/O

// The fd parameter needs to be a register.
// Uses X0-X3, X16.
// Return code is in X0.

// The all important syscall file
// in x86 assembly, there is the built-in syscall method
// not here...
#include <sys/syscall.h>

