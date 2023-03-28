#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub ditto);

mod ast;

fn main() {
    let val = ditto::GameParser::new().parse("game test").unwrap();
    println!("{:?}", val);
}

#[test]
fn ditto() {
    assert!(ditto::GameParser::new().parse("game test").is_ok());
}
