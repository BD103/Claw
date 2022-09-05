pub use claw_stdlib::EnumKind;

/// Represents a stage or sprite.
pub struct Target {
    /// The name of the target. If [`None`] then it is the stage, else it is a sprite.
    pub name: Option<String>,

    /// A list of [`Function`]s within the target.
    pub functions: Vec<Function>,

    /// A list of variables within the target.
    pub variables: Vec<String>,
}

impl Target {
    pub fn new_sprite(name: String) -> Self {
        Target {
            name: Some(name),
            ..Self::new_stage()
        }
    }

    pub fn new_stage() -> Self {
        Target {
            name: None,
            functions: Vec::new(),
            variables: Vec::new(),
        }
    }
}

/// Represents a single function of a [`Target`].
pub struct Function {
    /// The name of the function.
    pub name: String,

    /// A list of events that trigger this function
    pub events: Vec<Event>,

    /// A list of the blocks this function executes.
    pub blocks: Vec<Block>,
}

/// Represents a single instruction to be executed in a function.
pub struct Block {
    /// The module to find the block.
    pub module: String,

    /// The name of the block.
    pub name: String,

    /// A list of arguments to be passed to the block.
    ///
    /// # Note
    ///
    /// This is currently a slice, but may change in the future.
    pub arguments: Vec<Argument>,
}

/// Represents a parameter for a [`Block`], [`Function`], etc.
pub enum Argument {
    /// A text literal, e.g. `"Hello, world!"`.
    Text(String),

    /// An integer literal, e.g. `8`.
    Integer(u64),

    /// A decimal literal, e.g. `3.14`.
    Decimal(f64),

    /// A boolean literal, e.g. `true` or `false`.
    Boolean(bool),

    /// An enum variant. See [`EnumKind`] for more info.
    Enum {
        /// The kind of enum.
        kind: EnumKind,

        /// The variant of the enum to access.
        variant: String,
    },
}

/// Represents an event that can trigger a function.
///
/// # Note
///
/// Eventually a lifetime generic will be added for event arguments. Do not depend on this enum
/// until it is considered stable.
pub enum Event {
    /// Triggers at the start of the program.
    FlagClicked,
}
