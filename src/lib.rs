//! Throwaway crate for g-swarm end-to-end eval.

pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

pub fn divide(a: i64, b: i64) -> i64 {
    a / b
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
    fn divides_positive() {
        assert_eq!(divide(6, 3), 2);
    }

    #[test]
    fn divides_signed() {
        assert_eq!(divide(-9, 3), -3);
    }
}
