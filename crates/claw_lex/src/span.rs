#[derive(Debug, Clone, Copy)]
pub struct Span {
    /// Inclusive.
    pub start: usize,
    /// Exclusive.
    pub end: usize,
}

impl Span {
    #[inline]
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

#[derive(Debug, Clone)]
pub struct SpanDebug {
    pub start_line: usize,
    pub end_line: usize,
    pub text: String,
}

impl SpanDebug {
    pub fn from_span(span: &Span, script: &str) -> Self {
        // Probably not <=, as Spans must be at least 1 character
        debug_assert!(
            span.start < span.end,
            "Given invalid span with improper start and end points."
        );

        let mut current_line = 1;
        let mut start_line = 0;
        let mut end_line = 0;

        // Each line is collected and disposed of if not inside of span
        let mut text = String::new();
        let mut collection_status = TextCollectorStatus::NotStarted;

        for (i, c) in script.chars().enumerate() {
            if c == '\n' {
                match collection_status {
                    TextCollectorStatus::NotStarted => text.clear(),
                    TextCollectorStatus::Done => break,
                    _ => {}
                }

                current_line += 1;

                if let TextCollectorStatus::NotStarted = collection_status {
                    text.clear();
                }
            }

            if i == span.start {
                start_line = current_line;
                collection_status = TextCollectorStatus::Running;
            }

            if i == span.end - 1 {
                end_line = current_line;
                collection_status = TextCollectorStatus::Done;
            }

            text.push(c);
        }

        debug_assert_ne!(
            start_line, 0,
            "Given invalid span that reaches outside script length."
        );
        debug_assert_ne!(
            end_line, 0,
            "Given invalid span that reaches outside script length."
        );

        SpanDebug {
            start_line,
            end_line,
            text,
        }
    }

    pub fn is_single_line(&self) -> bool {
        self.start_line == self.end_line
    }
}

enum TextCollectorStatus {
    NotStarted,
    Running,
    Done,
}
