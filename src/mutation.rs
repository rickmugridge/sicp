/*#[derive(Debug, PartialEq, Clone)]
struct Environ {
    x: i32,
    y: i32,
}

impl Environ {
    fn f(&mut self, inc: i32) {
        self.y += inc;
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Pair {
    a: Expr22,
    b: Expr22,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr22 {
    IntAtom(isize),
    Symbol(Box<Pair>),
}

impl Expr22 {
    fn eval<'a>(&'a self, env: &'a mut Environ) -> Result<Expr22, &'a str> {
        env.f(1);
        env.f(1);
        match self {
            Expr22::IntAtom(i) => Ok(Expr22::IntAtom(*i)),
            Expr22::Symbol(s) => Expr22::pair(s, env)
        }
    }

/* The following error is due to &'a str part. Fixed it when changed to a String
error[E0499]: cannot borrow `*env` as mutable more than once at a time
  --> src/mutation.rs:37:60
   |
35 |     fn pair<'a>(s: &'a Box<Pair>, env: &'a mut Environ)-> Result<Expr22, &'a str>{
   |             -- lifetime `'a` defined here
36 |         Ok(Expr22::Symbol(
37 |             Box::new(Pair { a: s.a.eval(env)?, b: s.b.eval(env)? }
   |                                --------------              ^^^ second mutable borrow occurs here
   |                                |        |
   |                                |        first mutable borrow occurs here
   |                                returning this value requires that `*env` is borrowed for `'a`
 */


    fn pair<'a>(s: &'a Box<Pair>, env: &'a mut Environ)-> Result<Expr22, &'a str>{
        Ok(Expr22::Symbol(
            Box::new(Pair { a: s.a.eval(env)?, b: s.b.eval(env)? }
            )))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutations() {
        let e = &mut Environ { x: 1, y: 2 };
        assert_eq!(f(e, 0), &Environ { x: 21, y: 2 });
        assert_eq!(f(e, 1), &Environ { x: 61, y: 2 });
        assert_eq!(f(e, 2), &Environ { x: 121, y: 2 });
    }

    #[test]
    fn exp22() {
        let environ = &mut Environ { x: 1, y: 2 };
        let exp = &Expr22::IntAtom(4);
        assert_eq!(exp.eval(environ), Ok(Expr22::IntAtom(4)));
    }

    #[test]
    fn mutations2() {
        let e = &mut Environ { x: 1, y: 2 };
        assert_eq!(f(e, 0), &Environ { x: 21, y: 2 });
        assert_eq!(f(e, 1), &Environ { x: 61, y: 2 });
        assert_eq!(f(e, 2), &Environ { x: 121, y: 2 });
    }

    #[test]
    fn boxing() {
        // let e = &Expr::IntAtom(23);
        // assert_eq!(&e.eval(), Ok(e));
        assert_eq!(*boxed(), "abc");
    }
}

fn f(m: &mut Environ, i: i32) -> &Environ {
    g(m);
    g(m);
    if i <= 0 { m } else { f(m, i - 1) }
}

fn g(m: &mut Environ) {
    m.x += 10;
}

fn boxed<'a>() -> Box<&'a str> {
    let s = "abc";
    Box::new(s)
}*/

