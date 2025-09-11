use crate::ast::literals::Lit;

/// A stream of tokens.
#[derive(Debug, Clone)]
pub struct TokenStream {
    pub tokens: Vec<TokenTree>,
}

/// A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`).
#[derive(Debug, Clone)]
pub enum TokenTree {
    /// A token stream surrounded by bracket delimiters.
    Group(Group),
    /// An identifier.
    Ident(String),
    /// A single punctuation character (`+`, `,`, `$`, etc.).
    Punct(Punct),
    /// A literal character (`'a'`), string (`"hello"`), number (`2.3`), etc.
    Literal(Lit),
}

/// A token stream surrounded by bracket delimiters.
#[derive(Debug, Clone)]
pub struct Group {
    pub delimiter: Delimiter,
    pub stream: TokenStream,
}

/// A single punctuation character (`+`, `,`, `$`, etc.).
#[derive(Debug, Clone)]
pub struct Punct {
    pub ch: char,
    pub spacing: Spacing,
}

/// Whether a punctuation character is followed by another punctuation character
/// or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    /// The punctuation character is followed by another punctuation character,
    /// so there is no whitespace between them.
    Joint,
    /// The punctuation character is not followed by another punctuation character,
    /// so there is whitespace between them.
    Alone,
}

/// A delimiter for a token stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    /// `( ... )`
    Parenthesis,
    /// `{ ... }`
    Brace,
    /// `[ ... ]`
    Bracket,
    /// An implicit delimiter.
    None,
}
