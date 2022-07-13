# My Little Container

A small container project to help me learn Go, based on:
- [minict](https://github.com/Ripolak/minict)
- [Build Your Own Container Using Less Than 100 Lines of
  Go](https://www.infoq.com/articles/build-a-container-golang/)
- [Master Go While Learning
  Containers](https://iximiuz.com/en/posts/master-go-while-learning-containers/)

----

First up, let's read about containers. The first article I fully read (after
running down the 'container' rabbit hole) was written by [Julia
Evans](https://jvns.ca/blog/2016/10/10/what-even-is-a-container/). I also took
a look at [this small
implementation](https://blog.lizzie.io/linux-containers-in-500-loc.html).

The first thing to notice is that containers are essentially a combination of a
couple new functionalities in the Linux kernel, which makes it not possible to
do on a purely Windows or Mac machine, i.e. we gotta use a VM or something else
to simulate two important Linux kernel features:
- *namespaces*
- *cgroups* 

These two features together help us isolate processes, giving us the basics of
a 'container'. As Julia says, "these features let you pretend you have
something like a virtual machine, except it's not a virtual machine at all,
it's just a process running in the same Linux kernel."

**Notes**: another analogy (or approach?) that I've read about describes
containers as something similar to 'chroot jail', in which the `chroot` command
is used to change the apparent root process so as to isolate an entire branch
of processes -- this isn't exactly what is happening in a 'modern' Linux
container (and lets not start talking about OCI or Docker just yet).

## namespaces

Namespaces are labels (8 different kinds) which are structured in a
hierarchical/tree-like manner.  Processes are associated with a namespace such
that they only can see or use resources which are associated with said label,
or descendant namespaces where applicable.

`namespaces` are used to group kernel objects into different sets that can be
accessed by specific process trees. For example, PID namespaces limit the view
of the process list to the processes within the namespace, so if you set it to
the root (PID=00001) the process sees all other processes, but you can set it
to only see a couple different 'branches' of the process tree - recall that PID
is carried up to direct parents and down the children, but not 'horizontally'.

**Note**: there are other types of namespaces, which restrict 'view' and access
in different ways (and for different things) for processes. This is one
difference with namespaces and chroot, along with the fact that a process which
retains chroot privilege can still traverse back up the filesystem namespace,
i.e. the process still refers to the full mount namespace.

## cgroups

Allow you to limit the resources which a group of processes have access to,
like memory, disk io, and cpu-time. This allows us to make sure whatever
processes are run are all within a certain 'scope'.

## other kernel tools

In addtion, there are also a couple other useful kernel tools:
 - `seccomp-bpf`: lets you filter which system calls your process can run.
 - `capabilities`: set some coarse limits on what uid 0 can do
 - `strlimit`: is another mechanism for limiting resource usage.

 Now, let's start by jumping into some code, starting with C in the file
 `contained.c` (taken from
 [this](https://blog.lizzie.io/linux-containers-in-500-loc.html) blog post).


