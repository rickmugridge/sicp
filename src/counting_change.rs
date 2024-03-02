static DENOMINATIONS: [i32; 5] = [1, 5, 10, 25, 50];

fn counting_change(amount: i32) -> usize {
    cc(amount, 5)
}

fn cc(amount: i32, coin_kinds: usize) -> usize {
    if amount == 0 {
        1
    } else if amount < 0 || coin_kinds == 0 {
        0
    } else {
        cc(amount, coin_kinds - 1) + cc(amount - DENOMINATIONS[coin_kinds - 1], coin_kinds)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib1() {
        assert_eq!(counting_change(0), 1);
        assert_eq!(counting_change(-1), 0);
        assert_eq!(counting_change(1), 1);
    }

    #[test]
    fn fib5() {
        assert_eq!(counting_change(5), 2);
    }

    #[test]
    fn fib10() {
        assert_eq!(counting_change(10), 4);
    }

    #[test]
    fn fib11() {
        assert_eq!(counting_change(11), 4);
    }

    #[test]
    fn fib50() {
        assert_eq!(counting_change(50), 50);
    }

    #[test]
    fn fib0() {
        assert_eq!(counting_change(100), 292);
    }
}