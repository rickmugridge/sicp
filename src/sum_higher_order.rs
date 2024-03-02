use std::ops::{Add, Mul, Range};
use num_traits::identities::One;

fn sum<S, T, Map, Next>(map: &Map, from: S, to: S, next: &Next) -> T
    where
        S: PartialOrd + Copy,
        T: Add + Add<Output=T> + Copy + Default,
        Map: Fn(S) -> T,
        Next: Fn(S) -> S
{
    if from > to {
        T::default()
    } else {
        map(from) + sum(map, next(from), to, next)
    }
}

fn sum_iter<S, T, Map, Next>(map: &Map, from: S, to: S, next: &Next) -> T
    where
        S: PartialOrd + Copy,
        T: Add + Add<Output=T> + Copy + Default,
        Map: Fn(S) -> T,
        Next: Fn(S) -> S
{
    let mut result = T::default();
    let mut x = from;
    while x <= to {
        result = result + map(x);
        x = next(x);
    }
    result
}


fn sum_range<S, T, Map, Next>(map: &Map, range: Range<S>, next: &Next) -> T
    where
        S: PartialOrd + Copy,
        T: Add + Add<Output=T> + Copy + Default,
        Map: Fn(S) -> T,
        Next: Fn(S) -> S
{
    let mut result = T::default();
    let mut x = range.start;
    while x <= range.end {
        result = result + map(x);
        x = next(x);
    }
    result
}

fn integral<Map>(map: &Map, from: f64, to: f64, dx: f64) -> f64 where Map: Fn(f64) -> f64 {
    let next = |x: f64| x + dx;
    sum(map, from + dx / 2.0, to, &next) * dx
}

fn product_range<Map, Next>(map: &Map, range: Range<f64>, next: &Next) -> f64
    where
        Map: Fn(f64) -> f64,
        Next: Fn(f64) -> f64
{
    let mut result = 1.0;
    let mut x = range.start;
    while x < range.end {
        result *= map(x);
        x = next(x);
    }
    result
}

fn product_range_mul_one<S, T, Map, Next>(map: &Map, range: Range<S>, next: &Next) -> T
    where
        S: PartialOrd + Copy,
        T: Mul + Mul<Output=T> + One + Copy + Default,
        Map: Fn(S) -> T,
        Next: Fn(S) -> S
{
    let mut result = T::one();
    let mut x = range.start;
    while x < range.end {
        result = result * map(x);
        x = next(x);
    }
    result
}

fn product_range_mul_one_recursive<S, T, Map, Next>(map: &Map, range: Range<S>, next: &Next) -> T
    where
        S: PartialOrd + Copy,
        T: Mul + Mul<Output=T> + One + Copy + Default,
        Map: Fn(S) -> T,
        Next: Fn(Range<S>) -> Range<S>
{
    if range.is_empty() {
        T::one()
    } else {
        map(range.start) * product_range_mul_one_recursive(map, next(range), next)
    }
}

fn accumulate<S, T, Map, Combine, Next>(initial: T, map: &Map, combine: &Combine, range: Range<S>, next: &Next) -> T
    where
        S: PartialOrd + Copy,
        T: Copy,
        Map: Fn(S) -> T,
        Combine: Fn(T, T) -> T,
        Next: Fn(Range<S>) -> Range<S>
{
    if range.is_empty() {
        initial
    } else {
        combine(map(range.start), accumulate(initial, map, combine, next(range), next))
    }
}

// With the following, we can apply any map(), filter(), etc to the iterator separately
fn accumulate_iterator<T, Combine, Next>(initial: T, combine: &Combine, next: &mut Next) -> T
    where
        T: Copy,
        Combine: Fn(T, T) -> T,
        Next: Iterator<Item=T>,
{
    match next.next() {
        None => initial,
        Some(n) => combine(n, accumulate_iterator(initial, combine, next))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_integers() {
        let identity = |n: isize| n;
        let inc = |n: isize| n + 1;
        assert_eq!(sum(&identity, 1, 10, &inc), 55);
        assert_eq!(sum_iter(&identity, 1, 10, &inc), 55);
    }

    #[test]
    fn sum_of_cubes() {
        let cube = |n: isize| n * n * n;
        let inc = |n: isize| n + 1;
        assert_eq!(sum(&cube, 1, 10, &inc), 3025);
        assert_eq!(sum_iter(&cube, 1, 10, &inc), 3025);
        assert_eq!(sum_range(&cube, 1..10, &inc), 3025);
    }

    #[test]
    fn pi_sum() {
        let pi_term = |x: isize| 1.0 / (x * (x + 2)) as f64;
        let inc = |x: isize| x + 4;
        assert_eq!(8.0 * sum(&pi_term, 1, 1000, &inc), 3.139592655589783);
        assert_eq!(8.0 * sum_iter(&pi_term, 1, 1000, &inc), 3.139592655589782);
    }

    #[test]
    fn integral_() {
        let cube = |x: f64| x * x * x;
        assert_eq!(integral(&cube, 0.0, 1.0, 0.01), 0.24998750000000042);
        assert_eq!(integral(&cube, 0.0, 1.0, 0.001), 0.249999875000001);
        assert_eq!(integral(&cube, 0.0, 1.0, 0.0001), 0.24999999874993412);
    }

    #[test]
    fn product_of_integers() {
        let identity = |n: f64| n;
        let inc = |n: f64| n + 1.0;
        assert_eq!(product_range(&identity, 1.0..11.0, &inc), 3628800.0);
        assert_eq!(product_range_mul_one(&identity, 1.0..11.0, &inc), 3628800.0);
    }

    #[test]
    fn factorial_with_product() {
        let identity = |n: usize| n;
        let inc = |n: usize| n + 1;
        assert_eq!(product_range_mul_one(&identity, 1..5, &inc), 24);
        assert_eq!(product_range_mul_one(&identity, 1..11, &inc), 3628800);
    }

    #[test]
    fn pi_with_product() {
        let identity = |n: usize| n;
        let inc = |n: usize| n + 1;
        assert_eq!(product_range_mul_one(&identity, 1..5, &inc), 24);
        assert_eq!(product_range_mul_one(&identity, 1..11, &inc), 3628800);
        let next_range = |r: Range<usize>| (r.start + 1)..r.end;
        assert_eq!(product_range_mul_one_recursive(&identity, 1..11, &next_range), 3628800);
    }

    #[test]
    fn accumulate_sum_of_integers() {
        let identity = |n: usize| n;
        let combine = |x, y| x + y;
        let next_range = |r: Range<usize>| (r.start + 1)..r.end;
        assert_eq!(accumulate(0, &identity, &combine, 1..11, &next_range), 55);
        assert_eq!(accumulate_iterator(0, &combine, &mut (1..11)), 55);
    }

    #[test]
    fn accumulate_product_of_integers() {
        let identity = |n: usize| n;
        let combine = |x, y| x * y;
        let next_range = |r: Range<usize>| (r.start + 1)..r.end;
        assert_eq!(accumulate(1, &identity, &combine, 1..11, &next_range), 3628800);
        assert_eq!(accumulate_iterator(1, &combine, &mut (1..11)), 3628800);
    }
}