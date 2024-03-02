use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Clone)]
enum Algebra {
    Constant(isize),
    Variable(String),
    Sum(Box<Algebra>, Box<Algebra>),
    Product(Box<Algebra>, Box<Algebra>),
    Power(Box<Algebra>, Box<Algebra>),
}

impl Algebra {
    fn new_sum(addend: Algebra, augend: Algebra) -> Algebra {
        match (addend.clone(), augend.clone()) {
            (Algebra::Constant(x), Algebra::Constant(y)) => Algebra::Constant(x + y),
            (Algebra::Constant(0), x) | (x, Algebra::Constant(0)) => x,
            _ => Algebra::Sum(Box::new(addend),
                              Box::new(augend))
        }
    }

    fn new_product(multiplier: Algebra, multiplicand: Algebra) -> Algebra {
        match (multiplier.clone(), multiplicand.clone()) {
            (Algebra::Constant(x), Algebra::Constant(y)) => Algebra::Constant(x * y),
            (Algebra::Constant(0), _) | (_, Algebra::Constant(0)) => Algebra::Constant(0),
            (x, Algebra::Constant(1)) | (Algebra::Constant(1), x) => x,
            _ => Algebra::Product(Box::new(multiplier),
                                  Box::new(multiplicand))
        }
    }

    fn new_power(base: Algebra, exponent: Algebra) -> Algebra {
        match exponent.clone() {
            Algebra::Constant(0) => Algebra::Constant(1),
            Algebra::Constant(1) => base.clone(),
            _ => Algebra::Power(Box::new(base),
                                Box::new(exponent))
        }
    }

    fn derive(&self, variable: &str) -> Algebra {
        match self {
            Algebra::Constant(_) => Algebra::Constant(0),
            Algebra::Variable(v) =>
                if v == variable { Algebra::Constant(1) } else { Algebra::Constant(0) }
            Algebra::Sum(addend, augend) =>
                Algebra::new_sum(addend.derive(variable),
                                 augend.derive(variable)),
            Algebra::Product(multiplier, multiplicand) =>
                Algebra::new_sum(
                    Algebra::new_product((**multiplier).clone(),
                                         multiplicand.derive(variable)),
                    Algebra::new_product(multiplier.derive(variable),
                                         (**multiplicand).clone())),
            Algebra::Power(base, exponent) =>
                Algebra::new_product(
                    Algebra::new_product(
                        (**exponent).clone(),
                        Algebra::new_power((**base).clone(),
                                           Algebra::new_sum((**exponent).clone(),
                                                            Algebra::Constant(-1)))),
                    Algebra::derive(base, variable),
                )
        }
    }
}

impl Add for Algebra {
    type Output = Algebra;

    fn add(self, rhs: Self) -> Self::Output {
        Algebra::new_sum(self, rhs)
    }
}

impl Mul for Algebra {
    type Output = Algebra;

    fn mul(self, rhs: Self) -> Self::Output {
        Algebra::new_product(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn constant(i: isize) -> Algebra { Algebra::Constant(i) }

    fn var(s: &str) -> Algebra { Algebra::Variable(s.to_string()) }

    #[test]
    fn constant_derivative() {
        assert_eq!(constant(10).derive("x"), constant(0));
    }

    #[test]
    fn variable_differs() {
        let a = var("y");
        assert_eq!(a.derive("x"), constant(0));
    }

    #[test]
    fn variable_same() {
        let a = var("x");
        assert_eq!(a.derive("x"), constant(1));
    }

    #[test]
    fn sum() {
        let a = var("x") + constant(15);
        assert_eq!(a.derive("x"), constant(1));
    }

    #[test]
    fn product() {
        let y = var("y");
        let a = var("x") * y.clone();
        assert_eq!(a.derive("x"), y);
    }

    #[test]
    fn mixed() {
        let a = (var("x") * var("y")) * (var("x") + constant(3));
        assert_eq!(a.derive("x"),
                   (var("x") * var("y")) + (var("y") * (var("x") + constant(3))),
        );
    }

    #[test]
    fn power() {
        let a = Algebra::new_power(var("x"), constant(3));
        assert_eq!(a.derive("x"),
                   constant(3) * Algebra::new_power(var("x"), constant(2)));
    }

    #[test]
    fn power2() {
        let a = Algebra::new_power(var("x"), var("y"));
        assert_eq!(a.derive("x"),
                   var("y") * Algebra::new_power(var("x"), var("y") + constant(-1)));
    }
}