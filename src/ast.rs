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
    pub template: String,
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
pub struct EnemyInstanceNode {
    pub name: String,
    pub enemy: String,
}

#[derive(Debug)]
pub struct RoomNode {
    pub name: String,
    pub enemies: Vec<EnemyInstanceNode>,
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
pub struct FuncNode {
    pub name: String,
    pub args: Vec<Box<ExprEnum>>,
}

#[derive(Debug)]
pub struct WhileNode {
    pub cond: Box<ExprEnum>,
    pub children: Vec<Box<StmtEnum>>,
}

#[derive(Debug)]
pub struct AssignVarNode {
    pub l: String,
    pub r: Box<ExprEnum>,
}

#[derive(Debug)]
pub struct AssignPropNode {
    pub l: PropertyNode,
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
pub enum StmtEnum {
    While(WhileNode),
    AssignVar(AssignVarNode),
    AssignProp(AssignPropNode),
    Declare(DeclareNode),
    Cond(CondNode),
    Func(FuncNode),
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
    Dice(DiceNode),
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

#[derive(Debug)]
pub struct DiceNode {
    pub num: i32,
    pub faces: i32,
}
