use crate::ast::*;

pub fn eval(node: DittoNode) {

}

fn eval_stmt(stmt: &StmtEnum) {
    match stmt {
        StmtEnum::While(node) => eval_while(node),
        _ => (),
    }
}

fn eval_while(node: &WhileNode) {
    while itob(eval_expr(&*node.cond)) {
        for stmt_box in &node.children {
            eval_stmt(&*stmt_box);
        }
    }
}

fn eval_expr(expr: &ExprEnum) -> i32 {
    match expr {
        ExprEnum::BinOp(node) => eval_bin_op(node),
        ExprEnum::UnaryOp(node) => eval_unary_op(node),
        ExprEnum::Int(val) => *val,

        // TODO: replace with variable lookup
        ExprEnum::Var(name) => 0,

        // TODO: Replace with property lookup
        ExprEnum::Prop(node) => 0,
        _ => 0,
    }
}

fn itob(i: i32) -> bool {
    i != 0
}

fn eval_bin_op(node: &BinOpNode) -> i32 {
    match node.op {
        BinOp::Add => eval_expr(&*node.l) + eval_expr(&*node.r),
        BinOp::Sub => eval_expr(&*node.l) - eval_expr(&*node.r),
        BinOp::Mul => eval_expr(&*node.l) * eval_expr(&*node.r),
        BinOp::Div => eval_expr(&*node.l) / eval_expr(&*node.r),
        BinOp::Mod => eval_expr(&*node.l) % eval_expr(&*node.r),
        BinOp::And => (itob(eval_expr(&*node.l)) && itob(eval_expr(&*node.r))) as i32,
        BinOp::Or => (itob(eval_expr(&*node.l)) || itob(eval_expr(&*node.r))) as i32,
        BinOp::Equal=> (eval_expr(&*node.l) == eval_expr(&*node.r)) as i32,
        BinOp::Gt => (eval_expr(&*node.l) > eval_expr(&*node.r)) as i32,
        BinOp::Lt => (eval_expr(&*node.l) < eval_expr(&*node.r)) as i32,
        BinOp::Gte => (eval_expr(&*node.l) >= eval_expr(&*node.r)) as i32,
        BinOp::Lte => (eval_expr(&*node.l) <= eval_expr(&*node.r)) as i32,
    }
}

fn eval_unary_op(node: &UnaryOpNode) -> i32 {
    match node.op {
        UnaryOp::Neg => -eval_expr(&*node.l),
        UnaryOp::Not => (!itob(eval_expr(&*node.l))) as i32,
    }
}
