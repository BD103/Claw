mod hir;

pub use crate::hir::HIR;

use claw_parse::AST;

pub fn create_hir(ast: AST) -> HIR {
    // TODO: don't unwrap here
    HIR::try_from(ast).unwrap()
}
