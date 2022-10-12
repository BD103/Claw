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
        body: atom::Body::new(Vec::new(), 14..16),
        span: 7..16,
    };

    let my_declare = Declaration::Declare {
        kind: atom::Ident::new("var".to_string(), 16..19),
        items: vec![DeclareItem {
            name: atom::Ident::new("x".to_string(), 19..20),
            value: Some(Expr::Boolean(true, 20..24)),
            span: 19..24,
        }],
        span: 16..24,
    };

    let _ = Declaration::Sprite {
        name: atom::Ident::new("scratch".to_string(), 0..7),
        declarations: vec![my_func, my_declare],
        span: 0..25,
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

#[test]
fn declare_item() {
    let _ = DeclareItem {
        name: atom::Ident::new("x".to_string(), 0..1),
        value: None,
        span: 0..2,
    };

    let _ = DeclareItem {
        name: atom::Ident::new("high_score".to_string(), 0..10),
        value: Some(Expr::Number("0".to_string(), 11..12)),
        span: 0..12,
    };
}
