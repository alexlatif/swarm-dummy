//! Throwaway crate for g-swarm end-to-end eval.
//! The `add` function is intentionally unimplemented so the test suite fails
//! until a worker implements it.

pub fn add(a: i64, b: i64) -> i64 {
    unimplemented!("TODO: implement add so the tests pass")
}

pub fn multiply(a: i64, b: i64) -> i64 { a * b }

pub fn modulo(a: i64, b: i64) -> i64 { a % b }

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
    fn multiplies_value() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 5), -10);
    }

    #[test]
    fn modulos_value() {
        assert_eq!(modulo(7, 3), 1);
        assert_eq!(modulo(10, 5), 0);
    }
}
