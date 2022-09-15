// TODO: Fix error handling

pub mod hir;
pub mod lir;
mod hooks;

#[cfg(feature = "schema")]
mod schema;

#[cfg(feature = "schema")]
pub use schema::create_schema;

pub use crate::{hir::HIR, lir::LIR};

use claw_parse::AST;

pub fn create_hir(ast: AST) -> HIR {
    HIR::try_from(ast).unwrap()
}

pub fn create_lir(hir: HIR) -> LIR {
    LIR::try_from(hir).unwrap()
}
