mod hir;

pub use crate::hir::HIR;

use claw_parse::AST;

pub fn create_hir(_ast: AST) -> HIR {
    HIR::default()
}
