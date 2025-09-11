/// A comment.
#[derive(Debug, Clone)]
pub enum Comment {
    /// A line comment: `// ...`
    Line(String),
    /// A block comment: `/* ... */`
    Block(String),
}

/// A top-level item in a Rust file.
#[derive(Debug, Clone)]
pub enum Item {
    /// A function item: `fn foo() { ... }`
    Fn(ItemFn),
}

/// A function item: `fn foo() { ... }`
#[derive(Debug, Clone)]
pub struct ItemFn {
    /// Comments that appear before the function.
    pub leading_comments: Vec<Comment>,
    /// The function signature.
    pub sig: Signature,
    /// The function body.
    pub block: Block,
    /// Comments that appear after the function.
    pub trailing_comments: Vec<Comment>,
}

/// A function signature.
#[derive(Debug, Clone)]
pub struct Signature {
    // The `fn` token would go here.
    /// The name of the function.
    pub ident: String,
    // For simplicity, we'll omit arguments and return type for now.
}

/// A block of code: `{ ... }`
#[derive(Debug, Clone)]
pub struct Block {
    /// Comments that appear at the beginning of the block, before any statements.
    pub leading_comments: Vec<Comment>,
    /// The statements within the block.
    pub stmts: Vec<Stmt>,
    /// Comments that appear at the end of the block, after all statements.
    pub trailing_comments: Vec<Comment>,
}

/// A statement.
#[derive(Debug, Clone)]
pub enum Stmt {
    /// An expression statement, like `2 + 2;`.
    /// For now, we assume all expression statements are terminated by a semicolon.
    Expr(Expr),
}

/// An expression.
#[derive(Debug, Clone)]
pub enum Expr {
    /// A literal expression, like `1`, `"hello"`.
    Lit(Lit),
}

/// A literal.
#[derive(Debug, Clone)]
pub enum Lit {
    /// A string literal: `"..."`
    Str(String),
    /// An integer literal: `123`
    Int(u64),
}

use std::fmt::{self, Write};

/// A trait for pretty-printing AST nodes.
pub trait PrettyPrint {
    /// Pretty-prints the node to the given formatter.
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result;
}

/// A formatter for pretty-printing.
pub struct Formatter<'a> {
    writer: &'a mut dyn Write,
    indent: usize,
}

impl<'a> Formatter<'a> {
    /// Creates a new formatter.
    pub fn new(writer: &'a mut dyn Write) -> Self {
        Self { writer, indent: 0 }
    }

    /// Indents the output by one level.
    pub fn indent(&mut self) {
        self.indent += 4;
    }

    /// Dedents the output by one level.
    pub fn dedent(&mut self) {
        self.indent -= 4;
    }

    /// Writes a string to the output, preceded by the current indentation.
    pub fn write_line(&mut self, s: &str) -> fmt::Result {
        for _ in 0..self.indent {
            self.writer.write_char(' ')?;
        }
        self.writer.write_str(s)?;
        self.writer.write_char('\n')
    }

    /// Writes a string to the output without a newline, but with indentation.
    pub fn write_indented(&mut self, s: &str) -> fmt::Result {
        for _ in 0..self.indent {
            self.writer.write_char(' ')?;
        }
        self.writer.write_str(s)
    }

    /// Writes a string to the output without a newline.
    pub fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writer.write_str(s)
    }
}

impl PrettyPrint for Comment {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Comment::Line(s) => fmt.write_line(&format!("//{}", s)),
            Comment::Block(s) => fmt.write_line(&format!("/*{}*/", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ast = Item::Fn(ItemFn {
            leading_comments: vec![
                Comment::Line(" A simple function.".to_string()),
            ],
            sig: Signature {
                ident: "foo".to_string(),
            },
            block: Block {
                leading_comments: vec![
                    Comment::Block(" An inner comment ".to_string()),
                ],
                stmts: vec![
                    Stmt::Expr(Expr::Lit(Lit::Int(42))),
                ],
                trailing_comments: vec![],
            },
            trailing_comments: vec![
                Comment::Line(" Trailing comment.".to_string()),
            ],
        });

        let expected_code = r#"// A simple function.
fn foo() {
    /* An inner comment */
    42;
}
// Trailing comment.
"#;
        let mut buf = String::new();
        let mut fmt = Formatter::new(&mut buf);
        ast.pretty_print(&mut fmt).unwrap();

        assert_eq!(buf, expected_code);
    }
}

impl PrettyPrint for Item {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Item::Fn(item_fn) => item_fn.pretty_print(fmt),
        }
    }
}

impl ItemFn {
    /// Pretty-prints the function to a string.
    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut fmt = Formatter::new(&mut buf);
        self.pretty_print(&mut fmt).unwrap();
        buf
    }
}

impl PrettyPrint for ItemFn {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print(fmt)?;
        }

        fmt.write_str("fn ")?;
        self.sig.pretty_print(fmt)?;
        fmt.write_str(" ")?;
        self.block.pretty_print(fmt)?;

        for comment in &self.trailing_comments {
            comment.pretty_print(fmt)?;
        }

        Ok(())
    }
}

impl PrettyPrint for Signature {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str(&self.ident)?;
        fmt.write_str("()")?; // No args for now
        Ok(())
    }
}

impl PrettyPrint for Block {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_line("{")?;
        fmt.indent();

        for comment in &self.leading_comments {
            comment.pretty_print(fmt)?;
        }

        for stmt in &self.stmts {
            stmt.pretty_print(fmt)?;
        }

        for comment in &self.trailing_comments {
            comment.pretty_print(fmt)?;
        }

        fmt.dedent();
        fmt.write_indented("}")?;
        // Add a newline if this is the last item
        fmt.write_str("\n")
    }
}

impl PrettyPrint for Stmt {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_indented("")?;
        match self {
            Stmt::Expr(expr) => {
                expr.pretty_print(fmt)?;
                fmt.write_str(";")?;
            }
        }
        fmt.write_str("\n")
    }
}

impl PrettyPrint for Expr {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Lit(lit) => lit.pretty_print(fmt),
        }
    }
}

impl PrettyPrint for Lit {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Lit::Str(s) => fmt.write_str(&format!("\"{}\"", s)),
            Lit::Int(i) => fmt.write_str(&i.to_string()),
        }
    }
}
