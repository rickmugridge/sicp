pub fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a.rem_euclid(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remainder() {
        assert_eq!(10isize.rem_euclid(3), 1);
    }

    #[test]
    fn gcd0() {
        assert_eq!(gcd(206, 0),206);
    }

    #[test]
    fn gcd1() {
        assert_eq!(gcd(206, 40), 2);
        assert_eq!(gcd(40, 206), 2); // one extra step
    }
}