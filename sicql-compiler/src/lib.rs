pub fn remove(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod compiler {
    use super::*;

    #[test]
    fn compiler_works() {
        let result = remove(2, 2);
        assert_eq!(result, 0);
    }
}
