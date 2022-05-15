#[derive(Debug)]
pub struct Tree {
    pub root: Option<Branch>,
}

pub type Branch = Box<Node>;

#[derive(Debug)]
pub struct Node {
    value: i32,
    left: Option<Branch>,
    right: Option<Branch>
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, left: None, right: None }
    }
}

impl From<Node> for Option<Branch> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: i32) {
        match &mut self.root {
            None =>  {
                self.root = Node::new(value).into();
            },
            Some(node) => {
                Tree::insert_recursive(node, value);
            }
        }
    }

    fn insert_recursive(node: &mut Branch, value: i32) {
        if value > node.value {
            match &mut node.right {
                None => {
                    node.right = Node::new(value).into();
                },
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        }
        if value < node.value {
            match &mut node.left {
                None => {
                    node.left = Node::new(value).into();
                },
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        }
    }
}