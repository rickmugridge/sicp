fn a(x: isize, y: isize) -> isize {
    match (x, y) {
        (_, 0) => 0,
        (0, y) => 2 * y,
        (_, 1) => 2,
        (x, y) => a(x - 1, a(x, y - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egs() {
        assert_eq!(a(1, 10), 1024);
        assert_eq!(a(2, 4), 65536);
        assert_eq!(a(3, 3), 65536);
    }

    #[test]
    fn a0() {
        assert_eq!(a(0, 1), 2);
        assert_eq!(a(0, 2), 4);
        assert_eq!(a(0, 3), 6);
        assert_eq!(a(0, 4), 8);
    }

    #[test]
    fn a1() {
        assert_eq!(a(1, 1), 2);
        assert_eq!(a(1, 2), 4);
        assert_eq!(a(1, 3), 8);
        assert_eq!(a(1, 4), 16);
    }

    #[test]
    fn a2() {
        assert_eq!(a(2, 1), 2);
        assert_eq!(a(2, 2), 4);
        assert_eq!(a(2, 3), 16);
        assert_eq!(a(2, 4), 65536);
        assert_eq!(2i64.pow(16), 65536);
        // assert_eq!(a(2,5), 5); stack overflow
    }
}