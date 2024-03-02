fn fib(n: usize) -> usize {
    let mut count = n;
    let mut a = 1;
    let mut b = 0;

    while count > 0 {
        let a_ = a;
        a += b;
        b = a_;
        count -= 1;
    }
    b
}

fn fib_recurse(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2)
    }
}

fn fast_fib_recurse(n: usize) -> usize {
    fast_fib_recurse2(1, 0, 0, 1, n)
}

fn fast_fib_recurse2(a: usize, b: usize, p: usize, q: usize, count: usize) -> usize {
    if count == 0 {
        b
    } else if count % 2 == 0 {
        fast_fib_recurse2(a, b, p * p + q * q, 2 * q * p + q * q, count / 2)
    } else {
        fast_fib_recurse2(b * q + a * q + a * p, b * p + a * q, p, q, count - 1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib0() {
        let results2: Vec<usize> = (0..20).map(|n| fib(n)).collect();
        assert_eq!(results2, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181]);
    }

    #[test]
    fn fib0_recurse() {
        let results2: Vec<usize> = (0..20).map(|n| fib_recurse(n)).collect();
        assert_eq!(results2, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181]);
    }

    #[test]
    fn fast_fib_recurse_() {
        let results2: Vec<usize> = (0..20).map(|n| fast_fib_recurse(n)).collect();
        assert_eq!(results2, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181]);
    }
}