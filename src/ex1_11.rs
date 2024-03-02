fn f(n: usize) -> usize {
    if n < 3 { n } else { f(n - 1) + f(n - 2) + f(n - 3) }
}

fn f_iter(n: usize) -> usize {
    if n < 3 { return n; }
    let mut a = 0;
    let mut b = 1;
    let mut c = 2;
    let mut count = n;
    while count >= 3 {
        count -= 1;
        let d = a + b + c;
        a = b;
        b = c;
        c = d;
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eg() {
        assert_eq!(f(0), 0);
        assert_eq!(f(1), 1);
        assert_eq!(f(2), 2);
        assert_eq!(f(3), 3);
        assert_eq!(f(4), 6);
        assert_eq!(f(5), 11);
        assert_eq!(f(6), 20);
        assert_eq!(f(7), 37);
    }

    #[test]
    fn eg_iter() {
        assert_eq!(f_iter(0), 0);
        assert_eq!(f_iter(1), 1);
        assert_eq!(f_iter(2), 2);
        assert_eq!(f_iter(3), 3);
        assert_eq!(f_iter(4), 6);
        assert_eq!(f_iter(5), 11);
        assert_eq!(f_iter(6), 20);
        assert_eq!(f_iter(7), 37);
    }
}