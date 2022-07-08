# Part One

This part is definitely simpler than the later ones, since we are using higher
level languages which call the built-in `libc` functions to help deal with the
special Apple Silicon architecture (aka M1 chip, aka ARM64 CPU instructions
with some extra stuff).

In descending order 'level':
 1. `readfile.js` - uses JavaScript with the Node runtime. Just type `node
 readfile.js`.
 2. `readfile.rs` - good ol' Rust. Compile with `rustc readfile.rs` (with a
 possible `-C opt-level=0` if you want to see without optimizations) and then
 run the binary `./readfile`
 3. `readfile_v1.c` - C with `stdio` to help deal with buffers.
 4. `readfile_v2.c` - C without `stdio` to help deal with buffers.

