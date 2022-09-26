use chumsky::Parser;
use claw_parse::*;

const SIMPLE: &str = include_str!("simple.claw");

#[test]
fn simple() {
    let parser = create_parser();
    let res = parser.parse(SIMPLE);

    // Debug errors
    println!("{:#?}", res);

    // Commented out for now
    // assert!(res.is_ok());
}
