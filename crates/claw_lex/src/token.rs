/// Represents a token in a script.
///
/// Tokens are very low-level, and are used to filter out weird characters and
/// indentation.\ Most represents singular characters, like how
/// [`ParanOpen`](Self::ParanOpen) means `(`. Some are more complicated and
/// store extra data, like how [`Decimal`](Self::Decimal) represents any
/// floating point number. Tokens do not verify that syntax is correct, only
/// that [`char`]s are in the right place.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// An indentifier (`Sprite`) or a keyword (`declare`). The value isn't
    /// verified yet, so really it represents any word not wrapped in quotes.
    IdentOrKeyword(String),
    /// A string (`"Hello, world!"`).
    Text(String),
    /// An integer with an optional sign (`3`, `-26`).
    Integer(i64),
    /// A floating point number (`3.1415`).
    Decimal(f64),

    /// Opening parantheses (`(`).
    ParanOpen,
    /// Closing parantheses (`)`).
    ParanClose,
    /// Opening brackets (`{`).
    BracketOpen,
    /// Closing brackets (`}`).
    BracketClose,

    Add,
    Sub,
    Mul,
    Div,

    Eq,
    NotEq,
    And,
    Or,

    Colon,
    DoubleColon,
    Semi,
    Comma,
    AtSign,
    EnumSign,
    Not,

    Whitespace,
    Unknown,
}
