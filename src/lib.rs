//! Throwaway crate for g-swarm end-to-end eval.
//! The `add` function is intentionally unimplemented so the test suite fails
//! until a worker implements it.

pub fn add(a: i64, b: i64) -> i64 {
    unimplemented!("TODO: implement add so the tests pass")
}

pub fn noop() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adds_signed() {
        assert_eq!(add(-4, 1), -3);
    }

    #[test]
    fn noop_does_nothing() {
        noop();
    }
}
