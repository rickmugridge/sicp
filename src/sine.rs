use std::f64::consts::PI;

fn sine(x: f64) -> f64 {
    if x.abs() <= 0.1 { x } else { poly(sine(x / 3.0)) }
}

fn poly(x: f64) -> f64 {
    3.0 * x - 4.0 * (x * x * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(sine(0.0), 0.0);
    }

    #[test]
    fn small_enough() {
        assert_eq!(sine(0.1), 0.1);
    }

    #[test]
    fn half() {
        assert_eq!(sine(0.5), 0.4796515524505468);
    }

    #[test]
    fn one() {
        assert_eq!(sine(1.0), 0.8415945650055845);
    }

    #[test]
    fn two() {
        assert_eq!(sine(2.0), 0.9085328851396501);
    }

    #[test]
    fn pi() {
        assert_eq!(sine(PI), -0.0007881745995330647);
    }

    #[test]
    fn third() {
        let mut x = 12.15;
        let mut count = 0;
        while x > 0.1 {
            x = x / 3.0;
            count += 1;
        }
        assert_eq!(count, 5);
    }
}