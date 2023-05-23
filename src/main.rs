#[macro_use] extern crate lalrpop_util;

use std::fs;

lalrpop_mod!(pub ditto);

mod ast;
// mod eval;
mod scope;
// mod entity;
// mod item;

use crate::scope::Cactus;

fn main() {
    match run_test() {
        Ok(s) => {
            let val = ditto::DittoParser::new().parse(&s).unwrap();
            println!("{:?}", val);
        },
        Err(_e) => (),
    }
    let scope = Cactus::new();
    // scope.add("Test".to_string(), 2);
    // scope.lookup("Test".to_string());
}

#[test]
fn ditto() {
    assert!(ditto::ExprParser::new().parse("2 + 2").is_ok());
}

fn run_test() -> Result<String, std::io::Error> {
    let code = fs::read_to_string("src/test_scripts/test1.ditto");
    return code;
}
