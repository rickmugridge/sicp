use std::cmp::Ordering;

fn search<Function>(f: &Function, negative_point: f64, positive_point: f64) -> f64
    where Function: Fn(f64) -> f64 {
    let midpoint = average(negative_point, positive_point);
    if close_enough(negative_point, positive_point) {
        midpoint
    } else {
        let test_value = f(midpoint);
        if test_value > 0.0 {
            search(f, negative_point, midpoint)
        } else if test_value < 0.0 {
            search(f, midpoint, positive_point)
        } else {
            midpoint
        }
    }
}

fn average(x: f64, y: f64) -> f64 {
    (x + y) / 2.0
}

fn close_enough(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}

fn half_interval<Function>(f: &Function, a: f64, b: f64) -> f64
    where Function: Fn(f64) -> f64 {
    let a_value: f64 = f(a);
    let b_value: f64 = f(b);
    match (a_value.total_cmp(&0.0), b_value.total_cmp(&0.0)) {
        (Ordering::Less, Ordering::Greater) => search(f, a, b),
        (Ordering::Greater, Ordering::Less) => search(f, b, a),
        (_, _) => panic!("not on each side of 0.0")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_() {
        assert_eq!(average(2.0, 2.2), 2.1);
        assert_eq!(average(2.0, 4.0), 3.0);
        assert_eq!(average(10.0, 14.0), 12.0);
    }

    #[test]
    fn half_interval_sine() {
        let sin = |x: f64| x.sin();
        assert_eq!(half_interval(&sin, 2.0, 4.0), 3.14111328125);
    }

    #[test]
    fn half_interval_poly() {
        let poly = |x: f64| x * x * x - 2.0 * x - 3.0;
        assert_eq!(half_interval(&poly, 1.0, 2.0), 1.89306640625);
    }
}