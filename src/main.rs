mod sqrt;
mod fibonacci;
mod counting_change;
mod pascal_triangle;
mod sine;
mod rational;
mod gcd;
mod cons_clone;
mod cons;
mod binary_tree_immutable;
mod binary_tree_mutable;
mod mutation;
mod lisp;
// mod lisp_current; // Last working on this
mod ackermann;
mod ex1_11;
mod exponential;
mod prime;
mod sum_higher_order;
mod roots_of_equation;
mod fixed_point;
mod newton;
// p121
mod derivative;
mod picture_language;
mod huffman;
mod complex;
mod complex_trait;
mod complex_messaging;
mod digital_circuit;

use picture_language::picture;


// Following is from https://stackoverflow.com/questions/62960584/do-mutable-references-have-move-semantics
fn main() {
    // let mut name = String::from("Charlie");
    // let x = &mut name;
    // let y = x;       // x has been moved
    // say_hello(y);
    // say_hello(y);       // but y has not been moved, it is still usable
    // change_string(y); // Same as change_string(&mut *y); through compiler implicit re-borrowing
    // change_string(y);
    picture::run_picture();
}

fn say_hello(s: &str) {
    println!("Hello {}", s);
}

fn change_string(s: &mut String) {
    s.push_str(" Brown");
}

#[cfg(test)]
mod tests {
    // So we can run all the tests at once
    #[test]
    fn t() {
        assert_eq!(true, true);
    }
}