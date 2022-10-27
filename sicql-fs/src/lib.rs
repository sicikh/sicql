pub fn mult(left: usize, right: usize) -> usize {
    left * right
}

#[cfg(test)]
mod fs {
    use super::*;

    #[test]
    fn fs_works() {
        let result = mult(2, 3);
        assert_eq!(result, 6);
    }
}
