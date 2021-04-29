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
}                                                                               

