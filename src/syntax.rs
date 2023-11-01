use std::{rc::Rc, marker::PhantomData};

pub struct Name(Rc<str>);

pub enum Expr {
    Var(Name),
    App(Box<Expr>, Vec<Expr>),
    Lambda(Vec<Pattern>, Box<Expr>),
    Literal(Literal),
    ListLiteral(Vec<Expr>),
    RecordLiteral(Vec<(Name, Expr)>),
    Binop(Box<Expr>, Binop, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Sequence(Vec<Statement>),
    Perform(Name, Vec<Expr>),
    Handle(Box<Expr>, Vec<(Name, Vec<Pattern>, Name, Expr)>),
    Match(Box<Expr>, Vec<(Pattern, Expr)>)
}

pub enum Statement {
    Let(Box<Pattern>, Box<Expr>),
    LetFun(Name, Vec<Pattern>, Box<Expr>),
    RunExpr(Box<Expr>)
}

pub enum Pattern {
    VarPat(Name),
    LiteralPat(Literal),
    ListPat(Vec<Pattern>),
    ConsPat(Box<Pattern>, Box<Pattern>),
    RecordPat(Vec<(Name, Pattern)>),
    OrPat(Box<Pattern>, Box<Pattern>),
    AsPat(Box<Pattern>, Name)
}

pub enum Literal {
    NilLit,
    FloatLit(f64),
    IntLit(i64),
    BoolLit(bool)
}

pub enum Binop {
    Add,
    Subtract,
    Multiply,
    Divide,
    Less,
    LessOrEqual,
    Equal,
    NotEqual,
    GreaterOrEqual,
    Greater,
    Cons,
    Concat
}

pub enum Value<'runtime> {
    Nil,
    Float(f64),
    Int(i64),
    Bool(bool),
    List(RcList<Rc<Value<'runtime>>>),
    Closure(PhantomData<&'runtime ()>),
    Continuation,
}

pub enum RcList<A> {
    Nil,
    Cons(A, Rc<RcList<A>>)
}


