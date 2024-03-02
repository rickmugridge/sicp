pub fn fixed_point<Function>(f: &Function, first_guess: f64) -> f64
    where Function: Fn(f64) -> f64 {
    try_fixed_point(f, first_guess)
}

fn try_fixed_point<Function>(f: &Function, guess: f64) -> f64
    where Function: Fn(f64) -> f64 {
    let next: f64 = f(guess);
    if close_enough(guess, next) { next } else { try_fixed_point(f, next) }
}

fn close_enough(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.0001
}

fn sqrt(x:f64) -> f64 {
    let f = |y: f64| (y+x/y)/2.0;
    fixed_point(&f, 1.0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_point_cos() {
        let cos = |x: f64| x.cos();
        assert_eq!(fixed_point(&cos, 1.0), 0.7390547907469174);
    }

    #[test]
    fn fixed_point_trig_eqn() {
        let f = |x: f64| x.sin() + x.cos();
        assert_eq!(fixed_point(&f, 1.0), 1.2586974741689445);
    }

    #[test]
    fn fixed_point_sqrt() {
        assert_eq!(sqrt(9.0), 3.000000001396984);
    }
}