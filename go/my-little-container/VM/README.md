# A basic 'podman machine' implementation

To run containers on Mac, we need a Linux environment (since containers are
basically a collection of Linux kernel commands to isolate and control
processes). As such, if I want to do everything from scratch, I need to make a
way to run the Linux kernel, through a VM

If this fails, I can also just use a QEMU virtualized Linux distribution, but
that wouldn't be the best, since I would  have to manually run the VM every
time I wanted to use my container... (then again, when will I ever use this
container machinery over something like Docker or Podman and other OCI tools?).

