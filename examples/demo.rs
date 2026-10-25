use symbdiff::{Env, MathExpr, SymbolTable, basic_functions::BasicFunctions};

fn main() {
    let mut symbols = SymbolTable::new();
    let x = symbols.intern("x");
    let y = symbols.intern("y");

    let expression = (y + x) * x + 2.0;
    let expr = MathExpr::function(BasicFunctions::Ln, &expression);
    let derivative = expr.differentiate(x);

    let mut env = Env::new();
    env.set(x, 1.0);
    env.set(y, 2.0);

    println!("{}", expression.display_with(&symbols));
    println!("{}", expr.display_with(&symbols));
    println!("eval {}", expr.eval(&env));
    println!("{}", derivative.display_with(&symbols));
    println!("derivative: {}", derivative.eval(&env));
}
