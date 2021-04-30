mod ast;
mod eval;
mod grammar;
mod typechecker;

use eval::Evaluator;
use typechecker::TypeChecker;

fn main() {
    let input = "
        let f = x: int -> y: int ->
        x + y + 2
        in 3 > f < 2
    ".to_string();
    let e = grammar::program(&input).unwrap();
    println!("ast: {:?}", e);
    let t = TypeChecker::new().typecheck(&e).unwrap();
    println!("type: {:?}", t);
    let v = Evaluator::new().eval(&e).unwrap();
    println!("value: {:?}", v);
}

