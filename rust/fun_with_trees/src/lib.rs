mod tree;
pub use crate::tree::Tree;

#[cfg(test)]
mod tests {
    use crate::Tree;
    #[test]
    fn works_build_tree() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);

        assert_eq!(tree.root.is_some(), true);
        println!("{:?}", tree);
    }
}  
