use std::{collections::HashSet, iter::{self, Iterator}};

pub type Pos = usize;                                                               
                                                                                
#[derive(Debug, PartialEq, Eq, Hash, Clone)]                                                                
pub struct Range(pub Pos, pub Pos);                                                         
                                                                                
#[derive(Debug, PartialEq, Clone)]
pub enum TypeAst {
    Int(Range),
    Bool(Range),
    Function(Box<TypeAst>, Box<TypeAst>),
}

#[derive(Debug, PartialEq, Clone)]                                                                
pub enum Ast {                                                                  
    Paren(Box<Ast>, Range),
    Var(String, Range),
    IntLiteral(i32, Range),                                                            
    BoolLiteral(bool, Range),
    Add(Box<Ast>, Box<Ast>), 
    Eq(Box<Ast>, Box<Ast>),
    And(Box<Ast>, Box<Ast>),
    Let { 
        name: String,
        right: Box<Ast>,
        body: Box<Ast>,
        range: Range
    },

    Function {
        input: (String, Range),
        input_type: TypeAst,
        ret: Box<Ast>,
    },
    LApp(Box<Ast>, Box<Ast>),
    RApp(Box<Ast>, Box<Ast>),
}                                                                               

impl Ast {
    pub fn direct_children(&self) -> Vec<&Self> {
        match self {
            Ast::Paren(e, _) => vec![e.as_ref()],
            Ast::Var(_, _) | Ast::IntLiteral(_, _) | Ast::BoolLiteral(_, _) => vec![],
            Ast::And(l, r) | Ast::Eq(l, r) | Ast::Add(l, r) |
            Ast::LApp(l, r) | Ast::RApp(l, r) => {
                vec![l.as_ref(), r.as_ref()]
            },
            Ast::Let { right, body, .. } => vec![right.as_ref(), body.as_ref()],
            Ast::Function { ret, .. } => vec![ret.as_ref()],
        }
    }

    pub fn all_children(&self) -> impl Iterator<Item=&Self> {
        let direct_children = self.direct_children();
        let children = direct_children.into_iter().map(Self::all_children).flatten();
        iter::once(self).chain(children)
    }

    pub fn free_variables(&self) -> HashSet<&String> {
        match self {
            Ast::Let { name, right, body, .. } => {
                body.free_variables().into_iter().filter(|x| x != &name)
                .chain(right.free_variables()).collect()
            },
            Ast::Function { input: (input_var, _), ret, .. } => {
                ret.free_variables().into_iter().filter(|x| x != &input_var).collect()
            },
            Ast::Var(name, _) => [name].iter().cloned().collect(),
            x => x.direct_children().into_iter().map(Self::free_variables).flatten().collect(),
        }
    }
}


