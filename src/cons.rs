// p153
use std::boxed::Box;

#[derive(Debug, PartialEq)]
enum List<'a, T> {
    Empty,
    NonEmpty(Cons<'a, T>),
}

impl<'a, T> List<'a, T> {
    fn cons(t: &'a T, tail: Self) -> Self {
        List::NonEmpty(Cons::new(t, tail))
    }

    fn one(t: &'a T) -> Self {
        List::cons(t, List::Empty)
    }

    fn from_array<'b: 'a>(a: &'b [T]) -> Self {
        let mut result: List<'b, T> = List::Empty;
        for value in a.iter().rev() {
            result = List::cons(value, result);
        }
        result
    }

    fn head(&self) -> Option<&'a T> {
        match self {
            List::Empty => None,
            List::NonEmpty(some) => Some(some.car),
        }
    }

    fn tail(&self) -> Option<&List<'a, T>> {
        match self {
            List::Empty => None,
            List::NonEmpty(some) => Some(&*some.cdr),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            List::Empty => true,
            List::NonEmpty(_) => false,
        }
    }

    /*    fn map<U>(&self, f: &dyn Fn(&'a T) -> &'a U) -> List<'a, U> {
            match self {
                List::Empty => { List::Empty }
                List::NonEmpty(c) => List::cons(f(c.car), c.cdr.map(f))
            }
        }
    */

    fn filter(&self, f: &dyn Fn(&'a T) -> bool) -> Self {
        match self {
            List::Empty => List::Empty,
            List::NonEmpty(c) =>
                if f(c.car) {
                    List::cons(c.car, c.cdr.filter(f))
                } else {
                    c.cdr.filter(f)
                }
        }
    }

    fn fold<U>(&self, base: U, op: &dyn Fn(&'a T, U) -> U) -> U {
        match self {
            List::Empty => base,
            List::NonEmpty(c) =>
                op(c.car, c.cdr.fold(base, op))
        }
    }

    fn nth(&self, i: isize) -> Option<&'a T> {
        if i <= 0 {
            self.head()
        } else if let Some(t) = self.tail() {
            t.nth(i - 1)
        } else {
            None
        }
    }

    fn length(&self) -> isize {
        match self {
            List::Empty => 0,
            List::NonEmpty(c) => 1 + c.cdr.length(),
        }
    }

    fn length_iter(&self) -> isize {
        let mut length = 0;
        let mut list = self;
        while let List::NonEmpty(c) = list {
            length += 1;
            list = &*c.cdr;
        }
        length
    }

    fn reverse(&self) -> Self {
        self.reverse_builder(List::Empty)
    }

    fn reverse_builder(&self, accum: List<'a, T>) -> Self {
        match self {
            List::Empty => accum,
            List::NonEmpty(c) => c.cdr.reverse_builder(List::cons(c.car, accum)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Cons<'a, T> {
    car: &'a T,
    cdr: Box<List<'a, T>>,
}

impl<'a, T> Cons<'a, T> {
    fn new(car: &'a T, cdr: List<'a, T>) -> Self {
        Self { car, cdr: Box::new(cdr) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_array() {
        let it = &List::from_array(&[&1, &2, &3, &4][..]);
        assert_eq!(it.length(), 4);
    }

    #[test]
    fn head() {
        let empty = List::<isize>::Empty;
        assert_eq!(&empty.head(), &None);
        let one = &List::one(&1);
        assert_eq!(&one.head(), &Some(&1));
    }

    #[test]
    fn tail() {
        let empty = List::<isize>::Empty;
        assert_eq!(&empty.tail(), &None);
        let one = &List::one(&1);
        assert_eq!(&one.tail(), &Some(&List::Empty));
    }

    #[test]
    fn is_empty() {
        let empty = List::<isize>::Empty;
        assert_eq!(&empty.is_empty(), &true);
        let one = &List::one(&1);
        assert_eq!(&one.is_empty(), &false);
    }

    // Cannot define map() because we can't create a reference to data created in the fn argument
    /*    #[test]
        fn map() {
            // let empty = List::<isize>::Empty;
            // assert_eq!(&empty.map(&|v| &(v + 1)), &List::Empty);
            let one = &List::one(&1);
            assert_eq!(&one.map(&|v| &(*v + 1)), &List::one(&2));
        }
    */
    #[test]
    fn filter() {
        let empty = List::<isize>::Empty;
        assert_eq!(&empty.filter(&|v| *v > 0), &List::Empty);
        let many = &List::from_array(&[-1, 2, -3, 4][..]);
        assert_eq!(&many.filter(&|v| *v < 0), &List::from_array(&[-1, -3][..]));
        assert_eq!(&many.filter(&|v| *v > 0), &List::from_array(&[2, 4][..]));
    }

    #[test]
    fn fold() {
        let empty = List::<isize>::Empty;
        assert_eq!(&empty.fold(0, &|t, u| t + u), &0);
        let many = &List::from_array(&[1, 2, 3, 4][..]);
        assert_eq!(&many.fold(0, &|t, u| t + u), &10);
        assert_eq!(&many.fold(1, &|t, u| t * u), &24);
    }

    #[test]
    fn nth() {
        let it = &List::cons(&0, List::one(&1));
        assert_eq!(&it.nth(0), &Some(&0));
        assert_eq!(&it.nth(1), &Some(&1));
        assert_eq!(&it.nth(2), &None);
    }

    #[test]
    fn length() {
        let empty = List::<isize>::Empty;
        assert_eq!(&empty.length(), &0);
        let many = &List::from_array(&[-1, 2, -3, 4][..]);
        assert_eq!(&many.length(), &4);
    }

    #[test]
    fn length_iter() {
        let empty = List::<isize>::Empty;
        assert_eq!(&empty.length_iter(), &0);
        let many = &List::from_array(&[-1, 2, -3, 4][..]);
        assert_eq!(&many.length_iter(), &4);
    }

    #[test]
    fn reverse() {
        let it = &List::from_array(&[1, 2, 3, 4][..]);
        assert_eq!(it.reverse(), List::from_array(&[4, 3, 2, 1][..]));
    }
}
