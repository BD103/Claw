pub mod atom;

use std::ops::Range;

pub type Span = Range<usize>;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Expr {
    Call {
        module: atom::Ident,
        name: atom::Ident,
        args: Vec<Expr>,
        span: Span,
    },
    Number(String, Span),
    // Note: span includes the parenthesese
    Text(String, Span),
    Boolean(bool, Span),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span() {
        let my_span: Span = 0..5;
        assert_eq!(my_span, my_span.clone());

        assert_eq!(my_span.start, 0);
        assert_eq!(my_span.end, 5);
    }

    #[test]
    fn declaration() {
        let my_func = Declaration::Func {
            name: atom::Ident::new("do_thing".to_string(), 7..14),
            decorators: Vec::new(),
            body: Vec::new(),
            span: 7..16,
        };

        let my_declare = Declaration::Declare {
            kind: atom::Ident::new("var".to_string(), 16..19),
            items: Vec::new(),
            span: 16..21,
        };

        let _ = Declaration::Sprite {
            name: atom::Ident::new("scratch".to_string(), 0..7),
            declarations: vec![my_func, my_declare],
            span: 0..22,
        };
    }

    #[test]
    fn expr() {
        let _ = Expr::Number("3.1415".to_string(), 0..6);
        let _ = Expr::Text("hi there!".to_string(), 0..11);
        let (_, _) = (Expr::Boolean(true, 0..4), Expr::Boolean(false, 0..5));

        let _ = Expr::Call {
            module: atom::Ident::new("motion".to_string(), 0..6),
            name: atom::Ident::new("goto".to_string(), 6..10),
            args: vec![
                Expr::Number("0".to_string(), 11..12),
                Expr::Number("100".to_string(), 13..16),
            ],
            span: 0..1,
        };
    }
}
