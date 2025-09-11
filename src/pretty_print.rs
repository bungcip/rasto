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

impl PrettyPrint for ExprArray {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("[")?;
        for (i, elem) in self.elems.iter().enumerate() {
            if i > 0 {
                fmt.write_str(", ")?;
            }
            elem.pretty_print(fmt)?;
        }
        fmt.write_str("]")
    }
}

impl PrettyPrint for ExprAsync {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("async ")?;
        self.block.pretty_print(fmt)
    }
}

impl PrettyPrint for ExprAwait {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        self.expr.pretty_print(fmt)?;
        fmt.write_str(".await")
    }
}

impl PrettyPrint for ExprBreak {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("break")
    }
}

impl PrettyPrint for ExprCall {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        self.func.pretty_print(fmt)?;
        fmt.write_str("(")?;
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                fmt.write_str(", ")?;
            }
            arg.pretty_print(fmt)?;
        }
        fmt.write_str(")")
    }
}

impl PrettyPrint for ExprCast {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        self.expr.pretty_print(fmt)?;
        fmt.write_str(" as ")?;
        fmt.write_str(&self.ty)
    }
}

impl PrettyPrint for ExprClosure {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("|")?;
        for (i, input) in self.inputs.iter().enumerate() {
            if i > 0 {
                fmt.write_str(", ")?;
            }
            fmt.write_str(input)?;
        }
        fmt.write_str("| ")?;
        self.body.pretty_print(fmt)
    }
}

impl PrettyPrint for ExprConst {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("const ")?;
        self.block.pretty_print(fmt)
    }
}

impl PrettyPrint for ExprContinue {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("continue")
    }
}

impl PrettyPrint for ExprField {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        self.expr.pretty_print(fmt)?;
        fmt.write_str(".")?;
        fmt.write_str(&self.member)
    }
}

impl PrettyPrint for ExprIndex {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        self.expr.pretty_print(fmt)?;
        fmt.write_str("[")?;
        self.index.pretty_print(fmt)?;
        fmt.write_str("]")
    }
}

impl PrettyPrint for ExprMatch {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("match ")?;
        self.expr.pretty_print(fmt)?;
        fmt.write_line(" {")?;
        fmt.indent();
        for arm in &self.arms {
            arm.pretty_print(fmt)?;
            fmt.write_line(",")?;
        }
        fmt.dedent();
        fmt.write_indented("}")
    }
}

impl PrettyPrint for Arm {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_indented(&self.pat)?;
        if let Some(guard) = &self.guard {
            fmt.write_str(" if ")?;
            guard.pretty_print(fmt)?;
        }
        fmt.write_str(" => ")?;
        self.body.pretty_print(fmt)
    }
}

impl PrettyPrint for ExprMethodCall {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        self.receiver.pretty_print(fmt)?;
        fmt.write_str(".")?;
        fmt.write_str(&self.method)?;
        fmt.write_str("(")?;
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                fmt.write_str(", ")?;
            }
            arg.pretty_print(fmt)?;
        }
        fmt.write_str(")")
    }
}

impl PrettyPrint for ExprParen {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("(")?;
        self.expr.pretty_print(fmt)?;
        fmt.write_str(")")
    }
}

impl PrettyPrint for ExprRange {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        if let Some(start) = &self.start {
            start.pretty_print(fmt)?;
        }
        self.limits.pretty_print(fmt)?;
        if let Some(end) = &self.end {
            end.pretty_print(fmt)?;
        }
        Ok(())
    }
}

impl PrettyPrint for RangeLimits {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            RangeLimits::HalfOpen => fmt.write_str(".."),
            RangeLimits::Closed => fmt.write_str("..="),
        }
    }
}

impl PrettyPrint for ExprRef {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("&")?;
        if self.is_mut {
            fmt.write_str("mut ")?;
        }
        self.expr.pretty_print(fmt)
    }
}

impl PrettyPrint for ExprReturn {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("return")?;
        if let Some(expr) = &self.expr {
            fmt.write_str(" ")?;
            expr.pretty_print(fmt)?;
        }
        Ok(())
    }
}

impl PrettyPrint for ExprStruct {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str(&self.path)?;
        fmt.write_line(" {")?;
        fmt.indent();
        for field in &self.fields {
            field.pretty_print(fmt)?;
            fmt.write_line(",")?;
        }
        fmt.dedent();
        fmt.write_indented("}")
    }
}

impl PrettyPrint for FieldValue {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_indented(&self.member)?;
        fmt.write_str(": ")?;
        self.value.pretty_print(fmt)
    }
}

impl PrettyPrint for ExprTuple {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("(")?;
        for (i, elem) in self.elems.iter().enumerate() {
            if i > 0 {
                fmt.write_str(", ")?;
            }
            elem.pretty_print(fmt)?;
        }
        fmt.write_str(")")
    }
}

impl PrettyPrint for File {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        for item in &self.items {
            item.pretty_print(fmt)?;
            fmt.write_line("")?;
        }
        Ok(())
    }
}

impl PrettyPrint for ExprLoop {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("loop ")?;
        self.body.pretty_print(fmt)
    }
}

impl PrettyPrint for ExprWhile {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("while ")?;
        self.cond.pretty_print(fmt)?;
        fmt.write_str(" ")?;
        self.body.pretty_print(fmt)
    }
}

impl PrettyPrint for ExprFor {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str("for ")?;
        fmt.write_str(&self.pat)?;
        fmt.write_str(" in ")?;
        self.expr.pretty_print(fmt)?;
        fmt.write_str(" ")?;
        self.body.pretty_print(fmt)
    }
}

impl PrettyPrint for ExprAssign {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        self.left.pretty_print(fmt)?;
        fmt.write_str(" = ")?;
        self.right.pretty_print(fmt)
    }
}

impl PrettyPrint for ExprMacroCall {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str(&self.ident)?;
        fmt.write_str("!")?;
        self.tokens.pretty_print(fmt)
    }
}

impl PrettyPrint for TokenStream {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        for (i, token) in self.tokens.iter().enumerate() {
            if i > 0 {
                fmt.write_str(" ")?;
            }
            token.pretty_print(fmt)?;
        }
        Ok(())
    }
}

impl PrettyPrint for TokenTree {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            TokenTree::Group(group) => group.pretty_print(fmt),
            TokenTree::Ident(ident) => fmt.write_str(ident),
            TokenTree::Punct(punct) => punct.pretty_print(fmt),
            TokenTree::Literal(lit) => lit.pretty_print(fmt),
        }
    }
}

impl PrettyPrint for Group {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        let (open, close) = match self.delimiter {
            Delimiter::Parenthesis => ("(", ")"),
            Delimiter::Brace => ("{", "}"),
            Delimiter::Bracket => ("[", "]"),
            Delimiter::None => ("", ""),
        };
        fmt.write_str(open)?;
        self.stream.pretty_print(fmt)?;
        fmt.write_str(close)
    }
}

impl PrettyPrint for Punct {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.write_str(&self.ch.to_string())?;
        if self.spacing == Spacing::Alone {
            // fmt.write_str(" ")?;
        }
        Ok(())
    }
}

impl PrettyPrint for ItemTrait {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print(fmt)?;
        }

        fmt.write_indented(&format!("trait {} ", self.ident))?;
        fmt.write_line("{")?;
        fmt.indent();

        for item in &self.items {
            item.pretty_print(fmt)?;
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

impl PrettyPrint for TraitItem {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            TraitItem::Fn(item_fn) => item_fn.pretty_print(fmt),
        }
    }
}

impl PrettyPrint for TraitItemFn {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print(fmt)?;
        }

        fmt.write_indented("fn ")?;
        self.sig.pretty_print(fmt)?;
        if let Some(block) = &self.block {
            fmt.write_str(" ")?;
            block.pretty_print(fmt)?;
            fmt.write_str("\n")?;
        } else {
            fmt.write_line(";")?;
        }


        for comment in &self.trailing_comments {
            comment.pretty_print(fmt)?;
        }

        Ok(())
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
            Item::Trait(item_trait) => item_trait.pretty_print(fmt),
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
            Expr::Loop(expr) => expr.pretty_print(fmt),
            Expr::While(expr) => expr.pretty_print(fmt),
            Expr::For(expr) => expr.pretty_print(fmt),
            Expr::Assign(expr) => expr.pretty_print(fmt),
            Expr::MacroCall(expr) => expr.pretty_print(fmt),
            Expr::Array(expr) => expr.pretty_print(fmt),
            Expr::Async(expr) => expr.pretty_print(fmt),
            Expr::Await(expr) => expr.pretty_print(fmt),
            Expr::Break(expr) => expr.pretty_print(fmt),
            Expr::Call(expr) => expr.pretty_print(fmt),
            Expr::Cast(expr) => expr.pretty_print(fmt),
            Expr::Closure(expr) => expr.pretty_print(fmt),
            Expr::Const(expr) => expr.pretty_print(fmt),
            Expr::Continue(expr) => expr.pretty_print(fmt),
            Expr::Field(expr) => expr.pretty_print(fmt),
            Expr::Index(expr) => expr.pretty_print(fmt),
            Expr::Match(expr) => expr.pretty_print(fmt),
            Expr::MethodCall(expr) => expr.pretty_print(fmt),
            Expr::Paren(expr) => expr.pretty_print(fmt),
            Expr::Range(expr) => expr.pretty_print(fmt),
            Expr::Reference(expr) => expr.pretty_print(fmt),
            Expr::Return(expr) => expr.pretty_print(fmt),
            Expr::Struct(expr) => expr.pretty_print(fmt),
            Expr::Tuple(expr) => expr.pretty_print(fmt),
        }
    }
}

impl PrettyPrint for Lit {
    fn pretty_print(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Lit::Str(s) => fmt.write_str(&format!("\"{}\"", s)),
            Lit::Int(i) => fmt.write_str(&i.to_string()),
            Lit::Bool(b) => fmt.write_str(&b.to_string()),
        }
    }
}
