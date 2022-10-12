pub mod atom;

use std::ops::Range;

pub type Span = Range<usize>;

pub enum Declaration {
    Func {
        name: atom::Ident,
        decorators: Vec<()>,
        body: Vec<()>,
        span: Span,
    },

    Declare {
        kind: atom::Ident,
        items: Vec<()>,
        span: Span,
    },

    Sprite {
        name: atom::Ident,
        declarations: Vec<Declaration>,
        span: Span,
    },
}
