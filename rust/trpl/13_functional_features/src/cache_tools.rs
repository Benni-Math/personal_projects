pub mod rc_cacher;
mod cacher;

pub use self::cacher::Cacher;


#[cfg(test)]
mod tests {
    use crate::cache_tools::Cacher;
    #[test]
    fn test_cacher() {
        let mut cacher = Cacher::new(|x| x);

        // Use of clone here isn't the best...
        let run1 = cacher.value(5).clone();
        let run2 = cacher.value(7).clone();

        assert_ne!(run1, run2);
    }

    use crate::cache_tools::rc_cacher::Cacher as RcCacher;
    #[test]
    fn test_rc_cacher() {
        let mut cacher = RcCacher::new(|x| x);

        // Use of Rc allows us to avoid cloning
        let run1 = cacher.value(5);
        let run2 = cacher.value(7);

        assert_ne!(run1, run2);
    }
}