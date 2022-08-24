use super::{tokenize, LexError, Token};

#[test]
fn single_characters() {
    const SCRIPT: &str = "(){} +-*/ :;,@$!";
    const OUTPUT: [Token; 14] = [
        Token::ParanOpen,
        Token::ParanClose,
        Token::BracketOpen,
        Token::BracketClose,
        Token::Add,
        Token::Sub,
        Token::Mul,
        Token::Div,
        Token::Colon,
        Token::Semi,
        Token::Comma,
        Token::AtSign,
        Token::EnumSign,
        Token::Not,
    ];

    let tokens = tokenize(SCRIPT.to_string()).unwrap();
    assert_eq!(tokens, OUTPUT);
}

#[test]
fn double_characters() {
    // Double colons '::'
    {
        let tokens = tokenize("::".to_string()).unwrap();
        assert_eq!(tokens, [Token::DoubleColon]);

        let tokens = tokenize(":::".to_string());

        if let LexError::TripleColon(_) = tokens.unwrap_err() {
            assert!(true);
        } else {
            assert!(false, "Error was not LexError::TripleColons.");
        }
    }

    // Eq '=='
    {
        let tokens = tokenize("==".to_string()).unwrap();
        assert_eq!(tokens, [Token::Eq]);

        // Remove when variable shorthand is added
        let tokens = tokenize("=".to_string());

        if let LexError::SingleEq(_) = tokens.unwrap_err() {
            assert!(true);
        } else {
            assert!(false, "Error was not LexError::SingleEq.");
        }
    }

    // NotEq '!='
    {
        let tokens = tokenize("!=".to_string()).unwrap();
        assert_eq!(tokens, [Token::NotEq]);
    }

    // And '&&'
    {
        let tokens = tokenize("&&".to_string()).unwrap();
        assert_eq!(tokens, [Token::And]);

        let tokens = tokenize("&".to_string());

        if let LexError::SingleAnd(_) = tokens.unwrap_err() {
            assert!(true);
        } else {
            assert!(false, "Error was not LexError::SingleAnd.");
        }
    }

    // Or '||'
    {
        let tokens = tokenize("||".to_string()).unwrap();
        assert_eq!(tokens, [Token::Or]);

        let tokens = tokenize("|".to_string());

        if let LexError::SingleOr(_) = tokens.unwrap_err() {
            assert!(true);
        } else {
            assert!(false, "Error was not LexError::SingleOr.");
        }
    }
}

#[test]
fn numbers() {
    const SCRIPT: &str =
        "0 0.1 -0.34 5 -200000 3.14159265358979323846264338327950288 9223372036854775807";
    const OUTPUT: [Token; 7] = [
        Token::Integer(0),
        Token::Decimal(0.1),
        Token::Decimal(-0.34),
        Token::Integer(5),
        Token::Integer(-200000),
        Token::Decimal(3.14159265358979323846264338327950288), // PI
        Token::Integer(9223372036854775807),                   // Largest i64
    ];

    let tokens = tokenize(SCRIPT.to_string()).unwrap();
    assert_eq!(tokens, OUTPUT);
}

#[test]
fn strings() {
    let tokens = tokenize("\"Hello, world!\" \"Woo yeah\"".to_string()).unwrap();
    assert_eq!(
        tokens,
        [
            Token::Text("Hello, world!".to_string()),
            Token::Text("Woo yeah".to_string())
        ]
    );

    // Error handling
    {
        // Newline
        let tokens = tokenize("\"New\nline\"".to_string());

        if let LexError::StringNewline(_) = tokens.unwrap_err() {
            assert!(true);
        } else {
            assert!(false, "Error was not LexError::StringNewline.");
        }

        // Windows newline
        let tokens = tokenize("\"New\r\nline\"".to_string());

        if let LexError::StringNewline(_) = tokens.unwrap_err() {
            assert!(true);
        } else {
            assert!(false, "Error was not LexError::StringNewline.");
        }

        // No closing quotation mark
        let tokens = tokenize("\"Test".to_string());

        if let LexError::NoClosingQuotationMark(_) = tokens.unwrap_err() {
            assert!(true);
        } else {
            assert!(false, "Error was not LexError::NoClosingQuotationMark.");
        }
    }

    // Escaping strings
    {
        // Escaping quotes
        let tokens = tokenize(
            "\"Hi, \\\"there\\\"\"".to_string(), // Hi, "there"
        )
        .unwrap();
        assert_eq!(tokens, [Token::Text("Hi, \"there\"".to_string())]);

        // Escaping newlines
        let tokens = tokenize(
            "\"Woo\\\nyes\\\r\nhi\"".to_string(), // Woo\nyes\r\nhi
        )
        .unwrap();
        assert_eq!(tokens, [Token::Text("Woo\nyes\r\nhi".to_string())]);
    }
}

#[test]
fn empty() {
    let tokens = tokenize("".to_string()).unwrap();
    assert_eq!(tokens, []);
}

#[test]
fn unknown() {
    let tokens = tokenize("#".to_string());

    if let LexError::UnkownToken('#', _) = tokens.unwrap_err() {
        assert!(true);
    } else {
        assert!(false, "Error was not LexError::UnknownToken('#', _).");
    }
}

#[test]
fn is_whitespace() {
    use super::is_whitespace;

    // Only really testing the largest suspects
    assert!(is_whitespace(' '));
    assert!(is_whitespace('\t'));
    assert!(is_whitespace('\r'));
}

#[test]
fn full_program() {
    const SCRIPT: &str = "\
declare Event {
    DoThing,
}

sprite Scratch {
    @flag_clicked
    fn start() {
        forever {
            motion::goto(0, 0);
            control::wait(0.5);
            self::move_random();
            control::wait(0.5);
        }
    }

    fn move_random() {
        motion::goto(operators::random(-200, 200), operators::random(-150, 150));
    }
}";
    let output = {
        use super::Token::*;
        [
            IdentOrKeyword("declare".to_string()),
            IdentOrKeyword("Event".to_string()),
            BracketOpen,
            IdentOrKeyword("DoThing".to_string()),
            Comma,
            BracketClose,
            IdentOrKeyword("sprite".to_string()),
            IdentOrKeyword("Scratch".to_string()),
            BracketOpen,
            AtSign,
            IdentOrKeyword("flag_clicked".to_string()),
            IdentOrKeyword("fn".to_string()),
            IdentOrKeyword("start".to_string()),
            ParanOpen,
            ParanClose,
            BracketOpen,
            IdentOrKeyword("forever".to_string()),
            BracketOpen,
            IdentOrKeyword("motion".to_string()),
            DoubleColon,
            IdentOrKeyword("goto".to_string()),
            ParanOpen,
            Integer(0),
            Comma,
            Integer(0),
            ParanClose,
            Semi,
            IdentOrKeyword("control".to_string()),
            DoubleColon,
            IdentOrKeyword("wait".to_string()),
            ParanOpen,
            Decimal(0.5),
            ParanClose,
            Semi,
            IdentOrKeyword("self".to_string()),
            DoubleColon,
            IdentOrKeyword("move_random".to_string()),
            ParanOpen,
            ParanClose,
            Semi,
            IdentOrKeyword("control".to_string()),
            DoubleColon,
            IdentOrKeyword("wait".to_string()),
            ParanOpen,
            Decimal(0.5),
            ParanClose,
            Semi,
            BracketClose,
            BracketClose,
            IdentOrKeyword("fn".to_string()),
            IdentOrKeyword("move_random".to_string()),
            ParanOpen,
            ParanClose,
            BracketOpen,
            IdentOrKeyword("motion".to_string()),
            DoubleColon,
            IdentOrKeyword("goto".to_string()),
            ParanOpen,
            IdentOrKeyword("operators".to_string()),
            DoubleColon,
            IdentOrKeyword("random".to_string()),
            ParanOpen,
            Integer(-200),
            Comma,
            Integer(200),
            ParanClose,
            Comma,
            IdentOrKeyword("operators".to_string()),
            DoubleColon,
            IdentOrKeyword("random".to_string()),
            ParanOpen,
            Integer(-150),
            Comma,
            Integer(150),
            ParanClose,
            ParanClose,
            Semi,
            BracketClose,
            BracketClose,
        ]
    };

    let tokens = tokenize(SCRIPT.to_string()).unwrap();
    assert_eq!(tokens, output);
}
