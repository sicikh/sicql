pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod engine {
    use super::*;

    #[test]
    fn engine_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
