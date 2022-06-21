// The goal of this lib is to implement a lot of  algebraic and mathematical
// tools that I know. I also want it to be able to interop with Python/Julia
// and other languages. Not sure how to currently achieve this...
mod  linalg;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
