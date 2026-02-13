use std::collections::HashMap;

use symbdiff::expr;

fn main() {
    let expr = expr!((pow((exp(x)), 3.0)));
    let x = String::from("x");
    let derivative = expr.derive(&x);
    let mut assignments = HashMap::new();
    assignments.insert(stringify!(x), 1.0);
    println!("{expr}");
    println!("eval {}", expr.exec(&assignments));
    println!("{derivative}");
    println!("derivative: {}", derivative.exec(&assignments));
}
