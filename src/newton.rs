// use crate::fixed_point;

const DX: f64 = 0.00001;

fn derive(g: &dyn Fn(f64) -> f64) -> Box<dyn Fn(f64) -> f64 + '_> {
    Box::new(move |x| (g(x + DX) - g(x)) / DX)
}

fn cube(x: f64) -> f64 {
    x * x * x
}

/*fn newton_transform<'a>(g: &'a dyn Fn(f64) -> f64) -> Box<dyn Fn(f64) -> f64 + 'a> {
    Box::new(move |x| x - g(x) / derive(g)(x))
}

fn newtons_method<'a>(g: &'a dyn Fn(f64) -> f64, guess: f64) -> f64 {
    fixed_point(newton_transform(g), guess)
}

pub fn fixed_point(f: Box<dyn Fn(f64) -> f64>, first_guess: f64) -> f64 {
    try_fixed_point(f, first_guess)
}

fn try_fixed_point(f: Box<dyn Fn(f64) -> f64>, guess: f64) -> f64 {
    let next: f64 = f(guess);
    if close_enough(guess, next) { next } else { try_fixed_point(f, next) }
}

fn close_enough(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.0001
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivative_of_cube() {
        assert_eq!(derive(&cube)(5.0), 75.00014999664018);
    }
}