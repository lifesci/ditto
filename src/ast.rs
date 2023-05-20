#[derive(Debug)]
pub struct DittoNode<'a> {
    pub game: GameNode<'a>,
    pub statblock: StatblockNode<'a>,
    pub player: EntityNode<'a>,
    pub enemies: Vec<EntityNode<'a>>,
    pub items: Vec<ItemTemplateNode<'a>>,
    pub rooms: Vec<RoomNode<'a>>,
}

#[derive(Debug)]
pub struct GameNode<'a> {
    pub name: &'a str,
}

#[derive(Debug)]
pub struct StatblockNode<'a> {
    pub stats: Vec<StatNode<'a>>,
}

#[derive(Debug)]
pub struct EntityNode<'a> {
    pub entity: EntityType,
    pub name: &'a str,
    pub stats: Vec<StatNode<'a>>,
    pub actions: Vec<ActionNode<'a>>,
    pub triggers: Vec<TriggerNode<'a>>,
}

#[derive(Debug)]
pub enum EntityType { Player, Enemy }

#[derive(Debug)]
pub struct StatNode<'a> {
    pub name: &'a str,
    pub val: Box<ExprEnum<'a>>,
}

#[derive(Debug)]
pub struct ItemTemplateNode<'a> {
    pub name: &'a str,
    pub attribs: Vec<&'a str>,
    pub args: Vec<&'a str>,
    pub actions: Vec<ActionNode<'a>>,
}

#[derive(Debug)]
pub struct ItemInstanceNode<'a> {
    pub name: &'a str,
    pub template: &'a str,
    pub attribs: Vec<&'a str>,
    pub args: Vec<Box<ExprEnum<'a>>>,
}

#[derive(Debug)]
pub struct DoorNode<'a> {
    pub name: &'a str,
    pub to: &'a str,
    pub reqs: Vec<&'a str>,
}

#[derive(Debug)]
pub struct EnemyInstanceNode<'a> {
    pub name: &'a str,
    pub enemy: &'a str,
}

#[derive(Debug)]
pub struct RoomNode<'a> {
    pub name: &'a str,
    pub enemies: Vec<EnemyInstanceNode<'a>>,
    pub items: Vec<ItemInstanceNode<'a>>,
    pub doors: Vec<DoorNode<'a>>,
}

#[derive(Debug)]
pub struct ActionNode<'a> {
    pub name: &'a str,
    pub targets: Box<ExprEnum<'a>>,
    pub children: Vec<Box<StmtEnum<'a>>>,
}

#[derive(Debug)]
pub struct TriggerNode<'a> {
    pub name: &'a str,
    pub actions: Box<ExprEnum<'a>>,
    pub children: Vec<Box<StmtEnum<'a>>>,
}

#[derive(Debug)]
pub struct PropertyNode<'a> {
    pub var: &'a str,
    pub prop: &'a str,
}

#[derive(Debug)]
pub struct FuncNode<'a> {
    pub name: &'a str,
    pub args: Vec<Box<ExprEnum<'a>>>,
}

#[derive(Debug)]
pub struct WhileNode<'a> {
    pub cond: Box<ExprEnum<'a>>,
    pub children: Vec<Box<StmtEnum<'a>>>,
}

#[derive(Debug)]
pub struct AssignNode<'a> {
    pub l: Box<AssignEnum<'a>>,
    pub r: Box<ExprEnum<'a>>,
}

#[derive(Debug)]
pub struct DeclareNode<'a> {
    pub l: &'a str,
    pub r: Box<ExprEnum<'a>>,
}

#[derive(Debug)]
pub struct IfNode<'a> {
    pub cond: Box<ExprEnum<'a>>,
    pub children: Vec<Box<StmtEnum<'a>>>,
}

#[derive(Debug)]
pub struct CondNode<'a> {
    pub main: IfNode<'a>,
    pub alt: Vec<IfNode<'a>>,
    pub fin: Vec<Box<StmtEnum<'a>>>,
}

#[derive(Debug)]
pub enum AssignEnum<'a> {
    Var(&'a str),
    Prop(PropertyNode<'a>),
}

#[derive(Debug)]
pub enum StmtEnum<'a> {
    While(WhileNode<'a>),
    Assign(AssignNode<'a>),
    Declare(DeclareNode<'a>),
    Cond(CondNode<'a>),
    Func(FuncNode<'a>),
}

#[derive(Debug)]
pub struct BinOpNode<'a> {
    pub op: BinOp,
    pub l: Box<ExprEnum<'a>>,
    pub r: Box<ExprEnum<'a>>,
}

#[derive(Debug)]
pub struct UnaryOpNode<'a> {
    pub op: UnaryOp,
    pub l: Box<ExprEnum<'a>>,
}

#[derive(Debug)]
pub enum ExprEnum<'a> {
    BinOp(BinOpNode<'a>),
    UnaryOp(UnaryOpNode<'a>),
    Int(i32),
    Var(&'a str),
    Prop(PropertyNode<'a>),
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
