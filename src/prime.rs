use std::time::Instant;
use rand::Rng;

fn prime_naive(n: usize) -> bool {
    find_divisor(n, 2) == n
}

// Following includes the solution to Ex 1.23.
fn find_divisor(n: usize, test_divisor: usize) -> usize {
    if test_divisor * test_divisor > n {
        n
    } else if n % test_divisor == 0 {
        test_divisor
    } else {
        find_divisor(n, if test_divisor == 2 { 3 } else { test_divisor + 2 }) // Adding the conditional slows it down
    }
}

fn prime_naive_iter(n: usize) -> bool {
    let square_root = (n as f64).sqrt() as usize;
    for prime in 2..(square_root + 1) {
        if n % prime == 0 {
            return false;
        }
    }
    true
}

fn exp_mod(base: usize, exponent: usize, m: usize) -> usize {
    if exponent == 0 {
        1
    } else if exponent % 2 == 0 {
        let partial = exp_mod(base, exponent / 2, m);
        (partial * partial) % m
    } else {
        (base * exp_mod(base, exponent - 1, m)) % m
    }
}

fn fermat_test(n: usize) -> bool {
    let a = rand::thread_rng().gen_range(1..n);
    exp_mod(a, n, n) == a
}

fn fast_prime(n: usize, times: usize) -> bool {
    for _t in 0..times {
        if !fermat_test(n) {
            return false;
        }
    }
    true
}

fn time_search_for_primes(from: usize, f: &dyn Fn(usize) -> bool, algorithm_name: &str) {
    let now = Instant::now();
    search_for_primes(from, f);
    println!("\nAlgorithm {algorithm_name} from {from} took {} microseconds", now.elapsed().as_micros())
}

fn search_for_primes(from: usize, f: &dyn Fn(usize) -> bool) {
    let mut primes = 0;
    let mut n = from + 1;
    while primes < 3 {
        if f(n) {
            println!("    Found prime of {n}");
            primes += 1;
        }
        n += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_true() {
        assert_eq!(prime_naive(3), true);
        assert_eq!(prime_naive(11), true);
        assert_eq!(prime_naive_iter(3), true);
        assert_eq!(prime_naive_iter(11), true);
        assert_eq!(fast_prime(3, 10), true);
        assert_eq!(fast_prime(11, 10), true);
    }

    #[test]
    fn prime_false() {
        assert_eq!(prime_naive(4), false);
        assert_eq!(prime_naive(125), false);
        assert_eq!(prime_naive_iter(4), false);
        assert_eq!(prime_naive_iter(125), false);
        assert_eq!(fast_prime(4, 10), false);
        assert_eq!(fast_prime(125, 10), false);
    }

    #[test]
    fn ex_1_21() {
        assert_eq!(find_divisor(199, 2), 199);
        assert_eq!(find_divisor(1999, 2), 1999);
        assert_eq!(find_divisor(19999, 2), 7);
    }

    #[test]
    fn ex_1_22() {
        let naive = |n| prime_naive(n);
        let naive_iter = |n| prime_naive_iter(n);
        let fast = |n| fast_prime(n, 10);
        let algorithms: Vec<(&str, Box<dyn Fn(usize) -> bool>)> =
            vec![
                ("naive", Box::new(naive)),
                ("naive_iter", Box::new(naive_iter)),
                ("fast", Box::new(fast))];
        for n in 3..10 {
            for alg in &algorithms {
                time_search_for_primes((10 as usize).pow(n), &alg.1, alg.0);
            }
        }
    }

    // For solution to ex 1.23, see find_divisor() above
}