#[derive(Debug)]
pub struct GameNode {
    pub name: String
}

pub struct PropertyNode {
    pub var: String,
    pub prop: String,
}

pub struct WhileNode {
    pub cond: Box<ExprEnum>,
    pub children: Vec<Box<StmtEnum>>,
}

pub struct AssignNode {
    pub l: Box<AssignEnum>,
    pub r: Box<ExprEnum>,
}

pub struct DeclareNode {
    pub l: String,
    pub r: Box<ExprEnum>,
}

pub struct IfNode {
    pub cond: Box<ExprEnum>,
    pub children: Vec<Box<StmtEnum>>,
}

pub struct CondNode {
    pub main: IfNode,
    pub alt: Vec<IfNode>,
    pub fin: Vec<Box<StmtEnum>>,
}

pub enum AssignEnum {
    Var(String),
    Prop(PropertyNode),
}

pub enum StmtEnum {
    While(WhileNode),
    Assign(AssignNode),
    Declare(DeclareNode),
    Cond(CondNode),
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
