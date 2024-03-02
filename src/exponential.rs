fn exponential(base: usize, n: usize) -> usize {
    if n == 0 { 1 } else { base * exponential(base, n - 1) }
}

fn exponential_iter(base: usize, n: usize) -> usize {
    let mut result = 1;
    let mut nn = n;
    while nn > 0 {
        result *= base;
        nn -= 1;
    }
    result
}

fn fast_exponentiation(base: usize, exponent: usize) -> usize {
    if exponent == 0 {
        1
    } else if exponent % 2 == 0 {
        let exp = exponential(base, exponent / 2);
        exp * exp
    } else {
        base * fast_exponentiation(base, exponent - 1)
    }
}

fn fast_exponential_iter(base_given: usize, exponent_given: usize) -> usize {
    let mut exponent = exponent_given;
    let mut base = base_given;
    let mut result = 1;
    while exponent > 0 {
        if exponent % 2 == 0 {
            base = base * base;
            exponent /= 2;
        } else {
            result *= base;
            exponent -= 1;
        }
    }
    result
}

fn multiply(a: usize, b: usize) -> usize {
    if b == 0 { 0 } else { a + multiply(a, b - 1) }
}

fn multiply_iter(a: usize, b: usize) -> usize {
    let mut result = 0;
    (0..b).for_each(|_| { result += a; });
    result
}

fn fast_multiply_iter(a: usize, b: usize) -> usize {
    let mut aa = a;
    let mut bb = b;
    let mut result = 0;
    while bb > 0 {
        if bb % 2 == 0 {
            aa *= 2;
            bb /= 2;
        } else {
            result += aa;
            bb -= 1;
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponential_() {
        assert_eq!(exponential(10, 0), 1);
        assert_eq!(exponential(10, 1), 10);
        assert_eq!(exponential(10, 2), 100);
        assert_eq!(exponential(10, 3), 1000);
        assert_eq!(exponential(10, 6), 1_000_000);
        assert_eq!(exponential(10, 9), 1_000_000_000);
    }

    #[test]
    fn exponential_iter_() {
        assert_eq!(exponential_iter(10, 0), 1);
        assert_eq!(exponential_iter(10, 1), 10);
        assert_eq!(exponential_iter(10, 2), 100);
        assert_eq!(exponential_iter(10, 3), 1000);
        assert_eq!(exponential_iter(10, 6), 1_000_000);
        assert_eq!(exponential_iter(10, 9), 1_000_000_000);
    }

    #[test]
    fn fast_exponential_() {
        assert_eq!(fast_exponentiation(10, 0), 1);
        assert_eq!(fast_exponentiation(10, 1), 10);
        assert_eq!(fast_exponentiation(10, 2), 100);
        assert_eq!(fast_exponentiation(10, 3), 1000);
        assert_eq!(fast_exponentiation(10, 6), 1_000_000);
        assert_eq!(fast_exponentiation(10, 9), 1_000_000_000);
    }

    #[test]
    fn fast_exponential_iter_() {
        assert_eq!(fast_exponential_iter(10, 0), 1);
        assert_eq!(fast_exponential_iter(10, 1), 10);
        assert_eq!(fast_exponential_iter(10, 2), 100);
        assert_eq!(fast_exponential_iter(10, 3), 1000);
        assert_eq!(fast_exponential_iter(10, 6), 1_000_000);
        assert_eq!(fast_exponential_iter(10, 7), 10_000_000);
        assert_eq!(fast_exponential_iter(10, 8), 100_000_000);
        assert_eq!(fast_exponential_iter(10, 9), 1_000_000_000);
    }

    #[test]
    fn multiply_() {
        assert_eq!(multiply(10, 5), 50);
        assert_eq!(multiply(103, 51), 5253);
    }

    #[test]
    fn multiply_iter_() {
        assert_eq!(multiply_iter(10, 5), 50);
        assert_eq!(multiply_iter(103, 51), 5253);
        assert_eq!(multiply_iter(51, 103), 5253);
    }

    #[test]
    fn fast_multiply_iter_() {
        assert_eq!(fast_multiply_iter(10, 5), 50);
        assert_eq!(fast_multiply_iter(103, 51), 5253);
        assert_eq!(fast_multiply_iter(51, 103), 5253);
    }
}
