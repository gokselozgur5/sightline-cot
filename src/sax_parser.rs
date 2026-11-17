//! High-throughput zero-copy SAX (Simple API for XML) streaming parser for Cursor-on-Target.

use std::fmt;

/// Errors produced during SAX tokenization and CoT event extraction.
#[derive(Debug, Clone, PartialEq)]
pub enum SaxError {
    UnexpectedEof,
    InvalidSyntax(String),
    UnclosedElement(String),
    MismatchedTag { expected: String, found: String },
    InvalidEntity(String),
    InvalidNumericValue(&'static str),
}

impl fmt::Display for SaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => write!(f, "unexpected end of XML stream"),
            Self::InvalidSyntax(s) => write!(f, "invalid XML syntax: {s}"),
            Self::UnclosedElement(tag) => write!(f, "unclosed XML tag: <{tag}>"),
            Self::MismatchedTag { expected, found } => {
                write!(
                    f,
