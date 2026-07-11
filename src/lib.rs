//! Throwaway crate for g-swarm end-to-end eval.
//! The `add` function is intentionally unimplemented so the test suite fails
//! until a worker implements it.

pub fn add(a: i64, b: i64) -> i64 {
    unimplemented!("TODO: implement add so the tests pass")
}

pub fn negate(x: i64) -> i64 {
    -x
}

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
    fn negates_value() {
        assert_eq!(negate(5), -5);
        assert_eq!(negate(-3), 3);
        assert_eq!(negate(0), 0);
    }
}
