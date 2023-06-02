#[macro_use] extern crate lalrpop_util;

use std::fs;

lalrpop_mod!(pub ditto);

mod ast;
mod scope;
mod eval;
// mod entity;
// mod item;

fn main() {
    match run_test() {
        Ok(s) => {
            let val = ditto::DittoParser::new().parse(&s).unwrap();
            println!("{:?}", val);
        },
        Err(_e) => (),
    }
}

#[test]
fn ditto() {
    assert!(ditto::ExprParser::new().parse("2 + 2").is_ok());
}

fn run_test() -> Result<String, std::io::Error> {
    let code = fs::read_to_string("src/test_scripts/test1.ditto");
    return code;
}
