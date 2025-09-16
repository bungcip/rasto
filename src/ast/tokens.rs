//! Defines the AST nodes for tokens and token streams.
//!
//! This module is primarily used for representing the input to macros. A `TokenStream`
//! is a sequence of `TokenTree`s, which can be identifiers, punctuation, literals,
//! or grouped token streams.

use crate::ast::literals::Lit;
use compact_str::CompactString;
use thin_vec::ThinVec;

/// A stream of tokens, representing the input to a macro.
#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream {
    /// The sequence of token trees in the stream.
    pub tokens: ThinVec<TokenTree>,
}

impl From<ThinVec<TokenTree>> for TokenStream {
    fn from(tokens: ThinVec<TokenTree>) -> Self {
        Self { tokens }
    }
}

/// A single token or a delimited sequence of token trees (e.g., `[1, (), ..]`).
#[derive(Debug, Clone, PartialEq)]
pub enum TokenTree {
    /// A token stream surrounded by delimiters (e.g., `(...)`, `[...]`, `{...}`).
    Group(Group),
    /// An identifier, such as `foo` or `bar`.
    Ident(CompactString),
    /// A single punctuation character, such as `+`, `,`, or `$`.
    Punct(Punct),
    /// A literal, such as a character (`'a'`), a string (`"hello"`), or a number (`2.3`).
    Literal(Lit),
}

/// A token stream surrounded by delimiters.
#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    /// The type of delimiter.
    pub delimiter: Delimiter,
    /// The token stream inside the delimiters.
    pub stream: TokenStream,
}

/// A single punctuation character (`+`, `,`, `$`, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Punct {
    /// The character itself.
    pub ch: char,
    /// Indicates if the character is followed by another punctuation character.
    pub spacing: Spacing,
}

/// Describes the spacing of a punctuation character in a token stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    /// The punctuation character is immediately followed by another punctuation
    /// character, with no whitespace in between (e.g., `->`, `::`).
    Joint,
    /// The punctuation character is followed by whitespace or a non-punctuation
    /// token (e.g., `+`, `,`).
    Alone,
}

/// A delimiter for a token stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Delimiter {
    /// Parentheses: `( ... )`.
    Parenthesis,
    /// Braces: `{ ... }`.
    Brace,
    /// Brackets: `[ ... ]`.
    Bracket,
    /// An implicit delimiter used in contexts where no explicit delimiter is present.
    #[default]
    None,
}

/// The `!` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bang;

/// The `,` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comma;

/// The `=>` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FatArrow;
