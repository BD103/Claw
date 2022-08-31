//! Represents blocks included with the Scratch standard library.

mod events;

/// Represents an argument to a block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgType {
    /// A string.
    String,

    /// A number.
    Number,

    /// A boolean.
    Boolean,

    /// An enum.
    Enum { kind: EnumKind },

    /// Represents an unsupported argument that makes the block impossible to use.
    ///
    /// This is essentially a placeholder until I figure out if an argument type is worth having
    /// its own enum, or another solution should be found.
    Unsupported,
}

/// Represents different enums that can be accessed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumKind {
    /// Accesses the local (sprite) and global (stage) variables.
    Variable,
}

pub trait Module: Clone + Copy {
    /// Returns variant from given identifier.
    fn from_ident(ident: &str) -> Option<Self>;

    /// Returns the opcode of the current variant.
    fn opcode(&self) -> &'static str;
}

pub trait BlockModule: Module {}

pub trait EventModule: Module {}
