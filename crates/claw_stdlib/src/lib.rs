//! Represents blocks included with the Scratch standard library.

mod looks;
mod motion;
// mod sound;
mod events;

pub use self::{looks::Looks, motion::Motion, events::Events};

/// Represents what kind of block it is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockType {
    /// Does not return anything. This is the default.
    Block,
    /// Represents an event hat block.
    Hat,
    /// Returns a number.
    Number,
    /// Returns a string.
    String,
    /// Returns a boolean.
    Boolean,
}

impl Default for BlockType {
    fn default() -> Self {
        Self::Block
    }
}

/// Represents an argument to a block.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArgType {
    /// A string.
    String,

    /// A number.
    Number,

    /// A boolean.
    Boolean,

    /// An enum of all sprites in the project.
    Sprite,

    /// An enum of all costumes in the current sprite.
    Costume,

    /// An enum of all backdrops in the project.
    Backdrop,

    /// An enum of all possible visual effects to apply to the current sprite.
    Effect,

    /// An enum of all keyboard keys.
    KeyCode,

    /// An enum of all defined broadcasts.
    Event,

    /// Represents an unsupported argument that makes the block impossible to use.
    ///
    /// This is essentially a placeholder until I figure out if an argument type is worth having
    /// its own enum, or another solution should be found.
    Unsupported,
}

/// Represents a module in the Scratch standard library.
///
/// This is meant to be implemented on an enum.
pub trait Module: Clone + Copy + PartialEq {
    /// Returns [`Some`] item for a block's name. If the block does not exist, returns [`None`].
    fn get_from<'a>(name: &'a str) -> Option<Self>;

    /// Returns the [`BlockType`] of a block.
    fn get_type(&self) -> BlockType;

    /// Returns all [`ArgType`]s a block requires.
    fn get_args(&self) -> &[ArgType];
}
