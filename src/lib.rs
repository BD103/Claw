//! Claw is a programming language that compiles into Scratch blocks, or SB3.

/// Re-imported from [`claw_parse`].
pub mod parse {
    pub use claw_parse::*;
}

// Re-imported from [`claw_ir`].
pub mod ir {
    pub use claw_ir::*;
}

/// Re-imported from [`claw_schema`].
pub mod schema {
    pub use claw_schema::*;
}

/// Re-imported from [`claw_stdlib`].
pub mod stdlib {
    pub use claw_stdlib::*;
}
