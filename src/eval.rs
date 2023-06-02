use crate::ast::*;
use crate::scope::Cactus;

pub fn eval(node: &DittoNode) {

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
            if (eval_if(&alt, scope)) {
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
