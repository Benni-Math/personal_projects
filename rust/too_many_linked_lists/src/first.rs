// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }
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