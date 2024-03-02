fn pascal_triangle(n: usize) -> String {
    let tri = triangle(n);
    let mut result = String::new();
    result.push('\n');
    for r in 0..tri.len() {
        let row = &tri[r];
        result.push_str(".".repeat(n - r - 1).as_str());
        result.push_str(row[0].to_string().as_str());
        for digit in row.iter().skip(1) {
            result.push('.');
            result.push_str(digit.to_string().as_str());
        }
        if r < tri.len() - 1 { result.push('\n'); }
    }
    result
}

fn triangle(n: usize) -> Vec<Vec<usize>> {
    match n {
        0 => { vec![] }
        1 => { vec![vec![1]] }
        _ => {
            let mut previous = triangle(n - 1);
            let last = previous.last().expect("The base case has > 0 elements");
            previous.push(make_row(last));
            previous
        }
    }
}

fn make_row(last_row: &Vec<usize>) -> Vec<usize> {
    let mut row = vec![1];
    for i in 0..(last_row.len() - 1) {
        row.push(last_row[i] + last_row[i + 1]);
    }
    row.push(1);
    row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle1() {
        assert_eq!(triangle(1), vec![vec![1]]);
        assert_eq!(pascal_triangle(1), "
1");
    }

    #[test]
    fn triangle2() {
        assert_eq!(triangle(2), vec![vec![1], vec![1, 1]]);
        assert_eq!(pascal_triangle(2),
                   "
.1
1.1");
    }

    #[test]
    fn triangle3() {
        assert_eq!(triangle(3), vec![vec![1], vec![1, 1], vec![1, 2, 1]]);
        assert_eq!(pascal_triangle(3),
                   "
..1
.1.1
1.2.1");
    }

    #[test]
    fn triangle4() {
        assert_eq!(triangle(4), vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]);
        assert_eq!(pascal_triangle(4),
                   "
...1
..1.1
.1.2.1
1.3.3.1");
    }

    #[test]
    fn triangle5() {
        assert_eq!(triangle(5), vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]]);
        assert_eq!(pascal_triangle(5),
                   "
....1
...1.1
..1.2.1
.1.3.3.1
1.4.6.4.1");
    }
}
