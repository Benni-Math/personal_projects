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
    next: List,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() ->  Self {
        List { head: Link::Empty }
    }
}