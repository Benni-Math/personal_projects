// First naive implementation
// based on 'too_many_linked_lists'

pub struct Tree {
    root: Branch,
}

type Branch = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Branch,
    right: Branch,
}