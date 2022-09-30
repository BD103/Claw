// If this isn't running when you execute test, make sure to enable the "serde" feature. This will be skipped if disabled.
// https://doc.rust-lang.org/stable/cargo/reference/cargo-targets.html#the-required-features-field

use insta::{glob, assert_ron_snapshot};
use std::fs::read_to_string;

#[test]
fn lexer() {
    use chumsky::Parser;
    use claw_parse::lex::*;

    let lexer = create_lexer();

    glob!("samples/*.claw", |path| {
        let res = lexer.parse(read_to_string(path).unwrap());

        assert!(res.is_ok());
        assert_ron_snapshot!(res.unwrap());
    });
}
