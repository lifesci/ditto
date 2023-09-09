use crate::ast::*;
use crate::scope::Cactus;
use crate::item::Item;
use std::collections::HashMap;
use rand::Rng;

pub fn eval(node: &DittoNode) {
    let name = &node.game.name;
    let statblock = eval_statblock(&node.statblock);
    // &node.player;
    // &node.enemies;
    let item_templates = eval_item_templates(&node.items);
    // &node.rooms;
}

fn eval_statblock(node: &StatblockNode) -> HashMap<String, i32> {
    let scope = Cactus::new();
    let mut stats = HashMap::new();
    for stat_node in &node.stats {
        stats.insert(stat_node.name.to_string(), eval_expr(&stat_node.val, &scope));
    }
    stats
}

fn eval_item_templates(nodes: &Vec<ItemTemplateNode>) -> HashMap<String, &ItemTemplateNode> {
    let mut templates = HashMap::new();
    for node in nodes {
        templates.insert(node.name.to_string(), node);
    }
    templates
}

fn eval_item_instance(node: &ItemInstanceNode, templates: &HashMap<String, &ItemTemplateNode>) -> Item {
    Item::new(node, templates)
}

fn eval_stmt(stmt: &StmtEnum, scope: &mut Cactus) {
    match stmt {
        StmtEnum::While(node) => eval_while(node, scope),
        StmtEnum::Declare(node) => eval_declare(node, scope),
        StmtEnum::AssignVar(node) => eval_assign_var(node, scope),
        StmtEnum::Cond(node) => eval_cond(node, scope),
        _ => (),
    }
}

fn eval_while(node: &WhileNode, scope: &mut Cactus) {
    scope.push();
    while itob(eval_expr(&*node.cond, scope)) {
        eval_stmts(&node.children, scope);
    }
    scope.pop();
}

fn eval_declare(node: &DeclareNode, scope: &mut Cactus) {
    scope.add(node.l.to_string(), eval_expr(&*node.r, scope));
}

fn eval_assign_var(node: &AssignVarNode, scope: &mut Cactus) {
    scope.update(node.l.to_string(), eval_expr(&*node.r, scope));
}

fn eval_cond(node: &CondNode, scope: &mut Cactus) {
    if !eval_if(&node.main, scope) {
        let mut entered_alt = false;
        for alt in &node.alt {
            if eval_if(&alt, scope) {
                entered_alt = true;
                break;
            }
        }
        if !entered_alt {
            eval_stmts(&node.fin, scope);
        }
    }
}

fn eval_if(node: &IfNode, scope: &mut Cactus) -> bool {
    let res = itob(eval_expr(&*node.cond, scope));
    if res {
        scope.push();
        eval_stmts(&node.children, scope);
        scope.pop();
    }
    res
}

fn eval_stmts(stmts: &Vec<Box<StmtEnum>>, scope: &mut Cactus) {
    for stmt_box in stmts {
        eval_stmt(&*stmt_box, scope);
    }
}

fn eval_expr(expr: &ExprEnum, scope: &Cactus) -> i32 {
    match expr {
        ExprEnum::BinOp(node) => eval_bin_op(node, scope),
        ExprEnum::UnaryOp(node) => eval_unary_op(node, scope),
        ExprEnum::Int(val) => *val,
        ExprEnum::Var(name) => match scope.lookup(name) {
            None => panic!("Unbound variable: {name}", name=name),
            Some(x) => x,
        },

        // TODO: Replace with property lookup
        ExprEnum::Prop(node) => 0,
        ExprEnum::Dice(node) => eval_dice(node),
        _ => 0,
    }
}

fn itob(i: i32) -> bool {
    i != 0
}

fn eval_bin_op(node: &BinOpNode, scope: &Cactus) -> i32 {
    match node.op {
        BinOp::Add => eval_expr(&*node.l, scope) + eval_expr(&*node.r, scope),
        BinOp::Sub => eval_expr(&*node.l, scope) - eval_expr(&*node.r, scope),
        BinOp::Mul => eval_expr(&*node.l, scope) * eval_expr(&*node.r, scope),
        BinOp::Div => eval_expr(&*node.l, scope) / eval_expr(&*node.r, scope),
        BinOp::Mod => eval_expr(&*node.l, scope) % eval_expr(&*node.r, scope),
        BinOp::And => (itob(eval_expr(&*node.l, scope)) && itob(eval_expr(&*node.r, scope))) as i32,
        BinOp::Or => (itob(eval_expr(&*node.l, scope)) || itob(eval_expr(&*node.r, scope))) as i32,
        BinOp::Equal=> (eval_expr(&*node.l, scope) == eval_expr(&*node.r, scope)) as i32,
        BinOp::Gt => (eval_expr(&*node.l, scope) > eval_expr(&*node.r, scope)) as i32,
        BinOp::Lt => (eval_expr(&*node.l, scope) < eval_expr(&*node.r, scope)) as i32,
        BinOp::Gte => (eval_expr(&*node.l, scope) >= eval_expr(&*node.r, scope)) as i32,
        BinOp::Lte => (eval_expr(&*node.l, scope) <= eval_expr(&*node.r, scope)) as i32,
    }
}

fn eval_unary_op(node: &UnaryOpNode, scope: &Cactus) -> i32 {
    match node.op {
        UnaryOp::Neg => -eval_expr(&*node.l, scope),
        UnaryOp::Not => (!itob(eval_expr(&*node.l, scope))) as i32,
    }
}

fn eval_dice(node: &DiceNode) -> i32 {
    let mut rng = rand::thread_rng();
    (0..node.num).map(|_| rng.gen_range(1..=node.faces)).fold(0, |acc, x| acc + x)
}
