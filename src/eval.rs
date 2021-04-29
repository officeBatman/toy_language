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
}

