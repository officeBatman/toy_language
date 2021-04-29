mod ast;
mod eval;
mod grammar;
mod typechecker;

use eval::Evaluator;
use typechecker::TypeChecker;

fn main() {
    let input = "let x = 1 + 2 in x and 3".to_string();
    let e = grammar::expr(&input).unwrap();
    println!("ast: {:?}", e);
    let t = TypeChecker::new().typecheck(&e).unwrap();
    println!("type: {:?}", t);
    let v = Evaluator::new().eval(&e).unwrap();
    println!("value: {:?}", v);
}

