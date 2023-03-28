#[derive(Debug)]
pub struct GameNode {
    pub name: String
}

pub struct PropertyNode {
    pub var: String,
    pub prop: String,
}

pub struct BinOpNode {
    pub op: BinOp,
    pub l: Box<ExprEnum>,
    pub r: Box<ExprEnum>,
}

pub struct UnaryOpNode {
    pub op: UnaryOp,
    pub l: Box<ExprEnum>,
}

pub enum ExprEnum {
    BinOp(BinOpNode),
    UnaryOp(UnaryOpNode),
    Int(i32),
    Float(f32),
    Bool(bool),
    Str(String),
    Var(String),
    Prop(PropertyNode),
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Equal,
    Gt,
    Lt,
    Gte,
    Lte,
}

pub enum UnaryOp {
    Neg,
    Not,
}
