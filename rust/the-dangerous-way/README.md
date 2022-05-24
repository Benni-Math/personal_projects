Code based on the blog post series [Learn Rust the Dangerous Way](http://cliffle.com/p/dangerust/).

To run the 'final' version, just use `cargo run --release 50000000` (or build and then run manually).
You may need to change the target cpu -- go to `.cargo/config` and check that when you run `rustc --print cfg` in your terminal that the targets are the same - if not, then change it before compiling and running.

To run the earlier versions, go into the folder:
```
cd src/
```
compile using:
```
rustc -C opt-level=3 -C target-cpu=native -C codegen-units=1 safe-nbody.rs -o safe-nbody
```
and then run:
```
./safe-nbody 50000000
```