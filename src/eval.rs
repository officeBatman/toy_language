use std::{
    collections::hash_map::HashMap,
    cell::RefCell,
};
use crate::ast::Ast;

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeError();

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug, Clone)]
pub enum Value {
    I32(i32),
    Bool(bool),
    Function { 
        input_var: String,
        ret: Ast, 
        captured: HashMap<String, Value>
    },
}

impl Value {
    pub fn and(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(*b1 && *b2)),
            _ => Err(RuntimeError()),
        }
    }

    pub fn eq(&self, other: &Self) -> Result<bool> {
        match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => Ok(b1 == b2),
            (Value::I32(i1), Value::I32(i2)) => Ok(i1 == i2),
            _ => Err(RuntimeError()),
        }
    }

    pub fn add(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Value::I32(i1), Value::I32(i2)) => Ok(Value::I32(*i1 + *i2)),
            _ => Err(RuntimeError()),
        }
    }
}

pub struct Evaluator {
    vars: RefCell<HashMap<String, Value>>,
}

impl Evaluator {
    pub fn new() -> Self { Self { vars: RefCell::new(HashMap::new()) } }

    pub fn get_var(&self, name: &str) -> Option<Value> {
        self.vars.borrow().get(name).cloned()
    }

    pub fn var<F, T>(&self, name: String, value: Value, f: F) -> T
        where F: FnOnce(&Self) -> T
    {
        self.vars.borrow_mut().insert(name.clone(), value);
        let ret = f(self);
        self.vars.borrow_mut().remove(&name);
        ret
    }

    pub fn var_many<'a, F, T, I>(&self, vars: I, f: F) -> T
        where F: FnOnce(&Self) -> T, I: Iterator<Item=(String, Value)>
    {
        let mut names = vec![];
        for (name, value) in vars {
            names.push(name.clone());
            self.vars.borrow_mut().insert(name, value);
        }
        let ret = f(self);
        for name in names {
            self.vars.borrow_mut().remove(&name);
        }
        ret
    }

    pub fn eval(&self, ast: &Ast) -> Result<Value> {
        match ast {
            Ast::Paren(e, _) => self.eval(e),
            Ast::Var(name, _) => self.get_var(name).ok_or(RuntimeError()),
            Ast::IntLiteral(i, _) => Ok(Value::I32(*i)),
            Ast::BoolLiteral(b, _) => Ok(Value::Bool(*b)),
            Ast::And(l, r) => {
                let l = self.eval(l.as_ref())?;
                let r = self.eval(r.as_ref())?;
                Value::and(&l, &r)
            }
            Ast::Eq(l, r) => {
                let l = self.eval(l.as_ref())?;
                let r = self.eval(r.as_ref())?;
                Value::eq(&l, &r).map(Value::Bool)
            }
            Ast::Add(l, r) => {
                let l = self.eval(l.as_ref())?;
                let r = self.eval(r.as_ref())?;
                Value::add(&l, &r)
            }
            Ast::Let { name, right, body, .. } => {
                self.var(name.clone(), self.eval(right.as_ref())?, move |ev| {
                    ev.eval(body.as_ref())
                })
            }
            Ast::Function { input: (input_var, _), ret, .. } => {
                let captured = 
                    ret.free_variables()
                    .into_iter()
                    .filter(|x| x != &input_var)
                    .map(|x| match self.get_var(x) {
                        Some(v) => Ok((x.clone(), v.clone())),
                        None => Err(RuntimeError()),
                    })
                    .collect::<Result<HashMap<_,_>>>()?;
                Ok(Value::Function { 
                    input_var: input_var.clone(),
                    ret: ret.as_ref().clone(),
                    captured,
                })
            }
            Ast::LApp(func, arg) | Ast::RApp(arg, func) => {
                let func = self.eval(func.as_ref())?;
                let arg = self.eval(arg.as_ref())?;
                match func {
                    Value::Function { input_var, ret, captured } => {
                        self.var_many(captured.into_iter(), move |ev| {
                            ev.var(input_var.clone(), arg, move |ev| {
                                ev.eval(&ret)
                            })
                        })
                    }
                    _ => Err(RuntimeError()),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::grammar;

    #[test]
    fn let_expr() {
        let ev = Evaluator::new();
        let res = ev.eval(&grammar::expr("let x = 1 + 2 in x = 3").unwrap()).unwrap();
        assert!(matches!(res, Value::Bool(true)));

        let ev = Evaluator::new();
        let res = ev.eval(&grammar::expr("let x = 5 + 2 in x = 3").unwrap()).unwrap();
        assert!(matches!(res, Value::Bool(false)));
    }

    #[test]
    fn function_expression() {
        let ev = Evaluator::new();
        let code = "(x: int -> x + 2)";
        let res = ev.eval(&grammar::expr(code).unwrap()).unwrap();
        assert!(matches!(res, Value::Function { .. } ));
    }

    #[test]
    fn application() {
        let ev = Evaluator::new();
        let code = "(x: int -> x + 2) < 3";
        let res = ev.eval(&grammar::expr(code).unwrap()).unwrap();
        assert!(matches!(res, Value::I32(5)));
    }

    #[test]
    fn closures() {
        let ev = Evaluator::new();
        let code = "(x: int -> (y: int -> x + y)) < 3 < 4";
        let res = ev.eval(&grammar::expr(code).unwrap()).unwrap();
        assert!(matches!(res, Value::I32(7)));
    }
}

