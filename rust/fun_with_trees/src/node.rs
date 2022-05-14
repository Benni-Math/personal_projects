pub type Branch = Box<Node>;

#[derive(Debug)]
pub struct Tree {
    pub root: Option<Branch>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub left: Option<Branch>,
    pub right: Option<Branch>
}
