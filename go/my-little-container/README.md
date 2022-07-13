# My Little Container

A small container project to help me learn Go, based on:
 - [minict](https://github.com/Ripolak/minict)
 - [Build Your Own Container Using Less Than 100 Lines of Go](https://www.infoq.com/articles/build-a-container-golang/)
 - [Master Go While Learning Containers](https://iximiuz.com/en/posts/master-go-while-learning-containers/)

 ----

 First up, let's read about containers. The first article I fully read (after
 running down the 'container' rabbit hole) was written by [Julia Evans](https://jvns.ca/blog/2016/10/10/what-even-is-a-container/). 

 The first thing to notice is that containers are essentially a combination of
 a couple new functionalities in the Linux kernel, which makes it not possible
 to do on a purely Windows or Mac machine, i.e. we gotta use a VM or something
 else to simulate two important Linux kernel features:
  - *namesspaces*
  - *cgroups*
These two features together help us isolate processes, giving us the basics of
a 'container'. As Julia says, "these features let you pretend you have
something like a virtual machine, except it's not a virtual machine at all,
it's just a process running in the same Linux kernel."

## namespaces

Namespaces are labels (8 different kinds) which are structured in a hierarchical/tree-like manner.
Processes are associated with a namespace such that they only can see or use resources which are associated with said label, or descendant namespaces where applicable.

