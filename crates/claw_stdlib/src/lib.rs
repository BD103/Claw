//! Represents blocks included with the Scratch standard library.

/// Represents different enums that can be accessed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumKind {
    /// Accesses the local (sprite) and global (stage) variables.
    Variable,
}
