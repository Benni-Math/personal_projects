// Initially layout of doubly-linkedlist with cursor
// Could also do it with a dummy node which circles around,
// but that is more difficult to implement in Rust (see 7.1 in the book)

// pub struct LinkedList<T> {
//     front: Link<T>,
//     back: Link<T>,
//     len: usize,
// }
// 
// type Link<T> = *mut Node<T>;
// 
// struct Node<T> {
//     front<T>,
//     back<T>,
//     elem: T,
// }

// This layout doesn't work with subtyping due to the *mut Node<T>
// So, here is the new layout, with variance in mind

use std::ptr::NonNull;
use std::marker::PhantomData;

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    // We semantically store values of T by-value
    _boo: PhantomData<T>,
}

// NonNull works by storing *const 
// but casting to *mut at the boundary of the API
// -- We could do something similar, but let's use Option<NonNull>...
type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

// -- Basics --------------------
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }
}
