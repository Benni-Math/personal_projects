# Reading Files the Hard Way

Some code inspired by the series of articles on
[fasterthanli.me](fasterthanli.me)

Since I'm on an M1 Mac, I can't quite replicate everything done. Part 1 is easy
enough, since everything is done with high level programming languages, but the
problem starts immediately with part 2, since Apple Silicon uses ARM64 CPU
instructions instead of x86, and the syscall format is slightly different. I've
almost figured it out thanks to
[HelloSilicon](https://github.com/below/HelloSilicon/tree/main/Chapter%2007).
The real issue is part 3, in which Amos uses `ftrace` to look at the inner
workings of the Linux kernel. Might try a VM or just dual booting Asahi Linux, we'll see.

---

Not sure what directory to put this project in, as its a bit of an
'all-around-er' which deals with multiple languages and different approaches,
but I'm putting it in `c-assembly` for now, since it is a bit of a 'low-level'
adventure, and we will be using both C and assembly.
