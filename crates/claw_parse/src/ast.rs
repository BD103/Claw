/// Represents a stage or sprite.
pub struct Target<'a> {
    /// The name of the target. If [`None`] then it is the stage, else it is a sprite.
    name: Option<&'a str>,

    /// A list of [`Function`]s within the target.
    functions: Vec<Function<'a>>,

    /// A list of variables within the target.
    variables: Vec<&'a str>,
}

/// Represents a single function of a [`Target`].
pub struct Function<'a> {
    /// The name of the function.
    name: &'a str,

    /// A list of events that trigger this function
    events: Vec<Event>,

    /// A list of the blocks this function executes.
    blocks: Vec<Block<'a>>,
}

/// Represents a single instruction to be executed in a function.
pub struct Block<'a> {
    /// The module to find the block.
    module: &'a str,

    /// The name of the block.
    name: &'a str,

    /// A list of arguments to be passed to the block.
    ///
    /// # Note
    ///
    /// This is currently a slice, but may change in the future.
    arguments: &'a [Argument<'a>],
}

/// Represents a parameter for a [`Block`], [`Function`], etc.
pub enum Argument<'a> {
    /// A text literal, e.g. `"Hello, world!"`.
    Text(&'a str),

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
        variant: &'a str,
    },
}

/// Represents different enums that can be accessed.
pub enum EnumKind {
    /// Accesses the local (sprite) and global (stage) variables.
    Variable,
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
