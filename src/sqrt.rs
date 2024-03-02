fn sqrt(guess: f32, x: f32) -> f32 {
    if good_enough(guess, x) {
        guess
    } else {
        sqrt((guess + (x / guess)) / 2.0, x)
    }
}

fn good_enough(guess: f32, x: f32) -> bool {
    (guess * guess - x).abs() < 0.001
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_9() {
        let result = sqrt(1.0, 9.0);
        assert_eq!(result, 3.0000916);
    }

    #[test]
    fn sqrt_2() {
        let result = sqrt(1.0, 2.0);
        assert_eq!(result, 1.4142157);
    }

    #[test]
    fn sqrt_1() {
        let result = sqrt(0.5, 1.0);
        assert_eq!(result, 1.0003049);
    }
}