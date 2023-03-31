#[derive(Debug)]
pub struct DittoNode {
    pub game: GameNode,
    pub statblock: StatblockNode,
    pub player: EntityNode,
    pub enemies: Vec<EntityNode>,
    pub items: Vec<ItemTemplateNode>,
    pub rooms: Vec<RoomNode>,
}

#[derive(Debug)]
pub struct GameNode {
    pub name: String,
}

#[derive(Debug)]
pub struct StatblockNode {
    pub stats: Vec<StatNode>,
}

#[derive(Debug)]
pub struct EntityNode {
    pub entity: EntityType,
    pub name: String,
    pub stats: Vec<StatNode>,
    pub actions: Vec<ActionNode>,
    pub triggers: Vec<TriggerNode>,
}

#[derive(Debug)]
pub enum EntityType { Player, Enemy }

#[derive(Debug)]
pub struct StatNode {
    pub name: String,
    pub val: Box<ExprEnum>,
}

#[derive(Debug)]
pub struct ItemTemplateNode {
    pub name: String,
    pub attribs: Vec<String>,
    pub args: Vec<String>,
    pub actions: Vec<ActionNode>,
}

#[derive(Debug)]
pub struct ItemInstanceNode {
    pub name: String,
    pub attribs: Vec<String>,
    pub args: Vec<Box<ExprEnum>>,
}

#[derive(Debug)]
pub struct DoorNode {
    pub name: String,
    pub to: String,
    pub reqs: Vec<String>,
}

#[derive(Debug)]
pub struct RoomNode {
    pub name: String,
    pub items: Vec<ItemInstanceNode>,
    pub doors: Vec<DoorNode>,
}

#[derive(Debug)]
pub struct ActionNode {
    pub name: String,
    pub targets: Box<ExprEnum>,
    pub children: Vec<Box<StmtEnum>>,
}

#[derive(Debug)]
pub struct TriggerNode {
    pub name: String,
    pub actions: Box<ExprEnum>,
    pub children: Vec<Box<StmtEnum>>,
}

#[derive(Debug)]
pub struct PropertyNode {
    pub var: String,
    pub prop: String,
}

#[derive(Debug)]
pub struct WhileNode {
    pub cond: Box<ExprEnum>,
    pub children: Vec<Box<StmtEnum>>,
}

#[derive(Debug)]
pub struct AssignNode {
    pub l: Box<AssignEnum>,
    pub r: Box<ExprEnum>,
}

#[derive(Debug)]
pub struct DeclareNode {
    pub l: String,
    pub r: Box<ExprEnum>,
}

#[derive(Debug)]
pub struct IfNode {
    pub cond: Box<ExprEnum>,
    pub children: Vec<Box<StmtEnum>>,
}

#[derive(Debug)]
pub struct CondNode {
    pub main: IfNode,
    pub alt: Vec<IfNode>,
    pub fin: Vec<Box<StmtEnum>>,
}

#[derive(Debug)]
pub enum AssignEnum {
    Var(String),
    Prop(PropertyNode),
}

#[derive(Debug)]
pub enum StmtEnum {
    While(WhileNode),
    Assign(AssignNode),
    Declare(DeclareNode),
    Cond(CondNode),
}

#[derive(Debug)]
pub struct BinOpNode {
    pub op: BinOp,
    pub l: Box<ExprEnum>,
    pub r: Box<ExprEnum>,
}

#[derive(Debug)]
pub struct UnaryOpNode {
    pub op: UnaryOp,
    pub l: Box<ExprEnum>,
}

#[derive(Debug)]
pub enum ExprEnum {
    BinOp(BinOpNode),
    UnaryOp(UnaryOpNode),
    Int(i32),
    Var(String),
    Prop(PropertyNode),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}
