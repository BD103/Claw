//! Contains the abstract syntax tree definitions.

/// Public type used to represent a complete program in AST form.
pub type AST = Vec<Declaration>;

/// Declares constants.
///
/// Certain items in Scratch cannot be created at runtime, like variables. Declarations are a way of
/// representing this.
#[derive(Clone, Debug)]
pub enum Declaration {
    /// A function declaration.
    ///
    /// # Example
    ///
    /// ```claw
    /// fn do_thing() {
    ///     ...
    /// }
    /// ```
    Func {
        /// The name of the function.
        name: String,
        /// The events that trigger the function.
        events: Vec<String>,
        /// The code that is run when the function is called.
        body: Vec<Statement>,
    },

    /// An enum declaration.
    ///
    /// # Example
    ///
    /// ```claw
    /// declare Var {
    ///     Score,
    /// }
    /// ```
    Declare {
        /// The kind of enum being declared.
        kind: String,
        /// The items declared within the enum.
        items: Vec<String>,
    },

    /// A sprite declaration.
    ///
    /// # Example
    ///
    /// ```claw
    /// sprite Scratch {
    ///     declare Var { TimesMeowed }
    ///
    ///     fn meow() {
    ///         ...
    ///     }
    /// }
    /// ```
    Sprite {
        /// The name of the sprite.
        name: String,
        /// The declarations within the sprite.
        declarations: Vec<Declaration>,
    },
}

/// A statement that does not return a value.
///
/// Currently the language only supports `Call`.
#[derive(Clone, Debug)]
pub enum Statement {
    /// A function call that does not return anything.
    ///
    /// In Scratch, this is the same as a standalone block.
    ///
    /// # Example
    ///
    /// ```claw
    /// looks::hide();
    /// ```
    Call {
        /// The module the function is from.
        module: String,
        /// The name of the function.
        name: String,
        /// The arguments passed to the function.
        args: Vec<Expression>,
    },
}

/// An expression to be used from within a [`Statement`].
#[derive(Clone, Debug)]
pub enum Expression {
    /// A function that returns a value.
    ///
    /// # Example
    ///
    /// ```claw
    /// ops::join("apple", "banana")
    /// ```
    Call {
        /// The module the function is from.
        module: String,
        /// The name of the function.
        name: String,
        /// The arguments passed to the function.
        args: Vec<Expression>,
    },

    /// A number, whether decimal or not.
    ///
    /// The actual number is kept in a String to preserve precision.
    ///
    /// # Example
    ///
    /// ```claw
    /// 108
    /// 3.14159
    /// ```
    Number(String),

    /// A string of text.
    ///
    /// # Example
    ///
    /// ```claw
    /// "Hello, world!"
    /// ```
    Text(String),

    /// A boolean.
    ///
    /// # Example
    ///
    /// ```claw
    /// true
    /// false
    /// ```
    Boolean(bool),
}
