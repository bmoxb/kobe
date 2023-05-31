#[derive(Debug)]
pub enum Stat {
    Function(Name, Vec<(Name, Type)>, Option<Type>, Vec<Stat>),
    Expr(Expr),
    Let(Name, Type, Expr),
    Assign(Name, Expr),
    AddAssign(Name, Expr),
    SubAssign(Name, Expr),
    MulAssign(Name, Expr),
    DivAssign(Name, Expr),
    Return(Expr),
    If(Expr, Vec<Stat>, Option<Vec<Stat>>),
    While(Expr, Vec<Stat>),
    For(Name, Expr, Vec<Stat>),
}

#[derive(Debug)]
pub enum Expr {
    Name(Name),
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
    Array(Vec<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Equiv(Box<Expr>, Box<Expr>),
    NotEquiv(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    GreaterThanOrEqual(Box<Expr>, Box<Expr>),
    LessThanOrEqual(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
    Not(Box<Expr>),
}

pub type Name = String;
pub type Type = String;
