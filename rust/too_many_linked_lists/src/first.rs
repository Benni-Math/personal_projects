// The idea from functional programming: 
//      List a = Empty | Elem a (List a)
// The naive Rust implementation:
//      pub enum List {
//          Empty,
//          Elem(i32, Box<List>),
//      }

// A slightly less naive implementation:
pub struct List {
    head: Link
}
struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

use std::mem;

impl List {
    pub fn new() ->  Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            // We have to use mem::replace b/c of borrow rules
            next: mem::replace(&mut self.head, Link::Empty),
            // If we did:
            //  next: self.head
            // The compiler would say 'cannot move out of borrowed content'
            // Why? Still not quite getting why...
            // see https://doc.rust-lang.org/nightly/nomicon/exception-safety.html
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Need a better understanding of mem::replace
        //  -- Makes self.head into Link::Empty and gives us
        //     sets the match variable to point (and gain ownership?)
        //     of (a copy of?) self.head
        match mem::replace(&mut self.head, Link::Empty) {
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
            Link::Empty => None,
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        //TODO: Need to understand 'while let'
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
        //TODO: Draw out what's happening with the pointers
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list
        assert_eq!(list.pop(), None);

        // Populate
        list.push(5);
        list.push(6);
        list.push(7);
        list.push(8);
        // 'Who do we appreciate?'

        // Checking removal
        assert_eq!(list.pop(), Some(8));
        assert_eq!(list.pop(), Some(7));

        // Push more
        list.push(1);
        list.push(2);

        // Check removal again
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        // Check exhaustion
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}