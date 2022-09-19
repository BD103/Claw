//! Contains logic for error handling.
//!
//! This is specifically meant to work with the [`ariadne`] crate.

use std::ops::Range;

use crate::AST;
use ariadne::{Color, Fmt, Label, Report, ReportBuilder, ReportKind, Source};
use chumsky::error::Simple;

/// Error type returned by the parsers.
pub type ParseError = Simple<char>;

/// Takes a [`ParseError`] and applies it's contents to an existing [`ReportBuilder`].
///
/// This function is used to transform a list of errors into one concise [`Report`].
fn apply_err(
    report: ReportBuilder<Range<usize>>,
    error: &ParseError,
) -> ReportBuilder<Range<usize>> {
    use chumsky::error::SimpleReason;

    match error.reason() {
        SimpleReason::Unclosed { span, delimiter } => report
            .with_message(format!(
                "Unclosed delimiter {}",
                delimiter.fg(Color::Yellow),
            ))
            .with_label(
                Label::new(span.clone())
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow),
                    ))
                    .with_color(Color::Yellow),
            )
            .with_label(
                Label::new(error.span())
                    .with_message(format!(
                        "Must be closed before this {}",
                        error
                            .found()
                            .map_or_else(|| "end of file".to_string(), ToString::to_string)
                            .fg(Color::Red)
                    ))
                    .with_color(Color::Red),
            ),
        SimpleReason::Unexpected => report
            .with_message(format!(
                "{}, expected {}",
                if error.found().is_some() {
                    "Found unexpected token in input"
                } else {
                    "Found unexpected end of input"
                },
                if error.expected().len() == 0 {
                    "something else".to_string()
                } else {
                    error
                        .expected()
                        .map(|expected| match expected {
                            Some(expected) => expected.to_string(),
                            None => "end of input".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ))
            .with_label(
                Label::new(error.span())
                    .with_message(format!(
                        "Unexpected token {}",
                        match error.found() {
                            Some(c) => c.to_string(),
                            None => "end of file".to_string(),
                        }
                        .fg(Color::Red)
                    ))
                    .with_color(Color::Red),
            ),
        SimpleReason::Custom(msg) => report.with_message(msg).with_label(
            Label::new(error.span())
                .with_message(format!("{}", msg.fg(Color::Red)))
                .with_color(Color::Red),
        ),
    }
}

/// Transforms a result to use [`ariadne`]'s [`Report`].
///
/// This function is often used on the result of a parser made with
/// [`create_parser`](crate::create_parser).
#[allow(clippy::missing_errors_doc)]
pub fn build_report(parsed: Result<AST, Vec<ParseError>>) -> Result<AST, Report> {
    match parsed {
        Ok(parsed) => Ok(parsed),
        Err(errors) => {
            let mut report = Report::build(ReportKind::Error, (), {
                // Get the character the errors start on
                errors
                    .first()
                    .expect("Matched Err without any errors. Please file a bug report!")
                    .span()
                    .start
            });

            for err in errors {
                report = apply_err(report, &err);
            }

            Err(report.finish())
        }
    }
}

/// Returns the [`Source`] of a string reference.
///
/// This function exists so that other crates do not have to directly depend on [`ariadne`].
pub fn get_source<S: AsRef<str>>(script: S) -> Source {
    Source::from(script)
}
