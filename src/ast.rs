pub type Pos = usize;                                                               
                                                                                
#[derive(Debug, PartialEq, Eq, Hash)]                                                                
pub struct Range(pub Pos, pub Pos);                                                         
                                                                                
#[derive(Debug, PartialEq)]                                                                
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
}                                                                               

