# Part Two

This part is a little more involved, since I had to figure out how to do the
assembly file syscalls in ARM64 with MacOS, aka Apple Silicon. Importantly,
there is the file `fileio.S` which contains macros to perform file I/O which is
used in the actual `readfile.S` -- these macros, and the general knowledge of
assembly, were taken from
[HelloSilicon](https://github.com/below/HelloSilicon).
