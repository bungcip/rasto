use crate::ast::*;
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

impl PrettyPrint for ExprBinary {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        self.left.pretty_print(fmt)?;
        fmt.write_str(" ")?;
        self.op.pretty_print(fmt)?;
        fmt.write_str(" ")?;
        self.right.pretty_print(fmt)
    }
}

impl PrettyPrint for BinOp {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            BinOp::Add => fmt.write_str("+"),
            BinOp::Sub => fmt.write_str("-"),
            BinOp::Mul => fmt.write_str("*"),
            BinOp::Div => fmt.write_str("/"),
        }
    }
}

impl PrettyPrint for ExprIf {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("if ")?;
        self.cond.pretty_print(fmt)?;
        fmt.write_str(" ")?;
        self.then_branch.pretty_print(fmt)?;
        if let Some(else_branch) = &self.else_branch {
            fmt.write_str(" else ")?;
            else_branch.pretty_print(fmt)?;
        }
        Ok(())
    }
}

impl PrettyPrint for ExprBlock {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        self.block.pretty_print(fmt)
    }
}

impl PrettyPrint for Item {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Item::Fn(item_fn) => item_fn.pretty_print(fmt),
            Item::Struct(item_struct) => item_struct.pretty_print(fmt),
            Item::Enum(item_enum) => item_enum.pretty_print(fmt),
            Item::Impl(item_impl) => item_impl.pretty_print(fmt),
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

        fmt.write_indented("fn ")?;
        self.sig.pretty_print(fmt)?;
        fmt.write_str(" ")?;
        self.block.pretty_print(fmt)?;

        for comment in &self.trailing_comments {
            comment.pretty_print(fmt)?;
        }

        Ok(())
    }
}

impl PrettyPrint for Field {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_indented(&self.ident)?;
        fmt.write_str(": ")?;
        fmt.write_str(&self.ty)
    }
}

impl PrettyPrint for Variant {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_indented(&self.ident)
    }
}

impl PrettyPrint for ItemStruct {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print(fmt)?;
        }

        fmt.write_indented(&format!("struct {} ", self.ident))?;
        fmt.write_line("{")?;
        fmt.indent();

        for field in &self.fields {
            field.pretty_print(fmt)?;
            fmt.write_line(",")?;
        }

        fmt.dedent();
        fmt.write_line("}")?;
        fmt.write_line("")?;

        for comment in &self.trailing_comments {
            comment.pretty_print(fmt)?;
        }

        Ok(())
    }
}

impl PrettyPrint for ItemEnum {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print(fmt)?;
        }

        fmt.write_indented(&format!("enum {} ", self.ident))?;
        fmt.write_line("{")?;
        fmt.indent();

        for variant in &self.variants {
            variant.pretty_print(fmt)?;
            fmt.write_line(",")?;
        }

        fmt.dedent();
        fmt.write_line("}")?;
        fmt.write_line("")?;

        for comment in &self.trailing_comments {
            comment.pretty_print(fmt)?;
        }

        Ok(())
    }
}

impl PrettyPrint for ItemImpl {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print(fmt)?;
        }

        fmt.write_indented(&format!("impl {} ", self.ident))?;
        fmt.write_line("{")?;
        fmt.indent();

        for fun in &self.fns {
            fun.pretty_print(fmt)?;
        }

        fmt.dedent();
        fmt.write_line("}")?;

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
        fmt.write_line(" {")?;
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
        fmt.write_indented("}")
    }
}

impl PrettyPrint for StmtLet {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_indented("let ")?;
        fmt.write_str(&self.ident)?;
        if let Some(ty) = &self.ty {
            fmt.write_str(": ")?;
            fmt.write_str(ty)?;
        }
        if let Some(expr) = &self.expr {
            fmt.write_str(" = ")?;
            expr.pretty_print(fmt)?;
        }
        fmt.write_str(";")
    }
}

impl PrettyPrint for Stmt {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Stmt::Expr(expr) => {
                fmt.write_indented("")?;
                expr.pretty_print(fmt)?;
                if !matches!(expr, Expr::If(_) | Expr::Block(_)) {
                    fmt.write_str(";")?;
                }
            }
            Stmt::Let(stmt_let) => {
                stmt_let.pretty_print(fmt)?;
            }
        }
        fmt.write_str("\n")
    }
}

impl PrettyPrint for Expr {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Lit(lit) => lit.pretty_print(fmt),
            Expr::Binary(expr) => expr.pretty_print(fmt),
            Expr::If(expr) => expr.pretty_print(fmt),
            Expr::Block(expr) => expr.pretty_print(fmt),
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
