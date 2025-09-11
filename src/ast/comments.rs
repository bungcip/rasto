/// A comment.
#[derive(Debug, Clone)]
pub enum Comment {
    /// A line comment: `// ...`
    Line(String),
    /// A block comment: `/* ... */`
    Block(String),
}
