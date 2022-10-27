use sicql_compiler::remove;
use sicql_engine::add;
use sicql_fs::mult;

fn main() {
    println!("Hello, world!");
    println!("{}, {}, {}", add(1, 2), remove(2, 1), mult(2, 3))
}

fn add_one(left: usize, right: usize) -> usize {
    left + right + 1
}

#[cfg(test)]
mod repl {
    use super::*;

    #[test]
    fn repl_works() {
        let result = add_one(2, 4);
        assert_eq!(result, 7);
    }
}