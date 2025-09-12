//! A pretty-printer for the Rust AST.
//!
//! This module provides a flexible and efficient way to format Rust code from an
//! Abstract Syntax Tree (AST). The implementation is based on the paper
//! "A Prettier Printer" by Philip Wadler, which describes a linear-time algorithm
//! for pretty-printing documents with layout constraints.
//!
//! The core of the pretty-printer is the `Printer` struct, which manages the
//! printing process. It uses a token-based approach, where the AST is first
//! converted into a sequence of `Token`s. These tokens represent strings,
//! potential line breaks, and grouping constructs. The printer then uses a
//! two-pass algorithm:
//!
//! 1. **Scan Pass**: The printer scans the tokens to determine the best layout
//!    by calculating the size of each token group. This pass decides whether
//!    a group should be printed on a single line or broken into multiple lines.
//!
//! 2. **Print Pass**: The printer iterates through the tokens again, this time
//!    writing the formatted output to a `Write` buffer. It uses the information
//     from the scan pass to insert line breaks and indentation where necessary.
//!
//! The `PrettyPrintV2` trait is implemented by all AST nodes that can be
//! pretty-printed. This trait provides a `pretty_print_v2` method that
//! converts the AST node into a sequence of tokens for the `Printer`.

use crate::ast::*;
use std::borrow::Cow;
use std::fmt::{self, Write};

/// The line width to aim for when formatting.
const LINE_WIDTH: isize = 100;

/// A large integer value used to represent an infinitely long line.
const INFINITY: isize = 0xffff;

/// The style of a break.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BreakStyle {
    /// A consistent break means that if the group is broken, all breaks within
    /// the group will be broken.
    Consistent,
    /// An inconsistent break means that breaks within the group can be broken
    /// independently.
    Inconsistent,
}

/// A token used by the pretty-printer.
pub enum Token<'a> {
    /// A string to be printed.
    String(Cow<'a, str>),
    /// A potential line break. If the line is too long, this will be replaced
    /// with a newline and indentation. Otherwise, it will be replaced with a
    /// space.
    Break {
        /// The number of spaces to print if the break is not taken.
        len: usize,
    },
    /// A hard line break that will always be printed as a newline.
    HardBreak,
    /// The beginning of a group of tokens.
    Begin {
        /// The style of the break.
        style: BreakStyle,
        /// The opening string of the group (e.g., `(`, `[`, `{`).
        open: &'a str,
    },
    /// The end of a group of tokens.
    End {
        /// The closing string of the group (e.g., `)`, `]`, `}`).
        close: &'a str,
    },
    /// A comment.
    Comment(Cow<'a, str>),
}

/// A trait for types that can be pretty-printed.
pub trait PrettyPrintV2 {
    /// Pretty-prints the value to the given printer.
    ///
    /// # Parameters
    ///
    /// - `printer`: The `Printer` to use for formatting.
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result;
}

impl<T: PrettyPrintV2 + ?Sized> PrettyPrintV2 for Box<T> {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        (**self).pretty_print_v2(printer)
    }
}

/// A pretty-printer for the Rust AST.
pub struct Printer<'a> {
    writer: &'a mut dyn Write,
    tokens: Vec<Token<'a>>,
    sizes: Vec<isize>,
    // Ring buffer for scan
    scan_buffer: Vec<(usize, isize)>,
    scan_buffer_head: usize,
    right_total: isize,
    // Print state
    margin: isize,
    space: isize,
    indent: usize,
    print_stack: Vec<(usize, bool, BreakStyle)>, // (indent, is_broken, style)
}

impl<'a> Printer<'a> {
    /// Creates a new printer that writes to the given writer.
    ///
    /// # Parameters
    ///
    /// - `writer`: The `Write` buffer to write the formatted output to.
    pub fn new(writer: &'a mut dyn Write) -> Self {
        Self {
            writer,
            tokens: Vec::new(),
            sizes: Vec::new(),
            scan_buffer: vec![(0, 0); 3 * LINE_WIDTH as usize],
            scan_buffer_head: 0,
            right_total: 0,
            margin: LINE_WIDTH,
            space: LINE_WIDTH,
            indent: 0,
            print_stack: Vec::new(),
        }
    }

    /// Adds a string to the printer's token stream.
    ///
    /// # Parameters
    ///
    /// - `s`: The string to add.
    pub fn string(&mut self, s: impl Into<Cow<'a, str>>) {
        let s = s.into();
        if !s.is_empty() {
            self.tokens.push(Token::String(s));
        }
    }

    /// Adds a potential line break to the token stream.
    pub fn break_(&mut self) {
        self.tokens.push(Token::Break { len: 1 });
    }

    /// Adds a hard line break to the token stream.
    pub fn hard_break(&mut self) {
        self.tokens.push(Token::HardBreak);
    }

    /// Begins a new group of tokens.
    ///
    /// # Parameters
    ///
    /// - `style`: The `BreakStyle` of the group.
    /// - `open`: The opening string of the group (e.g., `(`, `[`, `{`).
    pub fn begin(&mut self, style: BreakStyle, open: &'a str) {
        self.tokens.push(Token::Begin { style, open });
    }

    /// Ends the current group of tokens.
    ///
    /// # Parameters
    ///
    /// - `close`: The closing string of the group (e.g., `)`, `]`, `}`).
    pub fn end(&mut self, close: &'a str) {
        self.tokens.push(Token::End { close });
    }

    /// Adds a comment to the token stream.
    ///
    /// # Parameters
    ///
    /// - `s`: The comment string.
    pub fn comment(&mut self, s: impl Into<Cow<'a, str>>) {
        self.tokens.push(Token::Comment(s.into()));
    }

    /// Scans the token stream to determine the best layout.
    ///
    /// This method implements the first pass of the pretty-printing algorithm.
    /// It calculates the size of each token group to decide whether it should
    /// be broken into multiple lines or printed on a single line.
    pub fn scan(&mut self) {
        self.sizes = vec![INFINITY; self.tokens.len()];
        self.scan_buffer_head = 0;
        self.right_total = 0;

        for i in 0..self.tokens.len() {
            match &self.tokens[i] {
                Token::Begin { .. } => {
                    self.scan_push(i, -self.right_total);
                }
                Token::End { .. } => loop {
                    if self.scan_buffer_head == 0 {
                        break;
                    }
                    self.scan_buffer_head -= 1;
                    let (j, offset) = self.scan_buffer[self.scan_buffer_head];
                    match self.tokens[j] {
                        Token::Begin { .. } => {
                            let len = self.right_total + offset;
                            self.sizes[j] = if len > self.margin { INFINITY } else { len };
                            break;
                        }
                        Token::Break { .. } | Token::HardBreak => {
                            let len = self.right_total + offset;
                            self.sizes[j] = if len > self.margin { INFINITY } else { len };
                        }
                        _ => {}
                    }
                },
                Token::Break { .. } | Token::HardBreak => {
                    while self.scan_buffer_head > 0 {
                        let (j, offset) = self.scan_buffer[self.scan_buffer_head - 1];
                        match self.tokens[j] {
                            Token::Begin { .. } => break,
                            Token::Break { .. } | Token::HardBreak => {
                                self.scan_buffer_head -= 1;
                                let len = self.right_total + offset;
                                self.sizes[j] = if len > self.margin { INFINITY } else { len };
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    self.scan_push(i, -self.right_total);
                }
                Token::String(s) => {
                    self.right_total += s.len() as isize;
                }
                Token::Comment(s) => {
                    self.right_total += s.len() as isize;
                }
            }
        }

        while self.scan_buffer_head > 0 {
            self.scan_buffer_head -= 1;
            let (j, offset) = self.scan_buffer[self.scan_buffer_head];
            if let Token::Break { .. } | Token::HardBreak = self.tokens[j] {
                let len = self.right_total + offset;
                self.sizes[j] = if len > self.margin { INFINITY } else { len };
            }
        }
    }

    fn scan_push(&mut self, i: usize, offset: isize) {
        self.scan_buffer[self.scan_buffer_head] = (i, offset);
        self.scan_buffer_head += 1;
    }

    /// Prints the token stream to the writer.
    ///
    /// This method implements the second pass of the pretty-printing algorithm.
    /// It iterates through the tokens and writes the formatted output to the
    /// `Write` buffer, using the layout information from the `scan` pass.
    pub fn print(&mut self) -> fmt::Result {
        for i in 0..self.tokens.len() {
            match &self.tokens[i] {
                Token::Begin { style, open } => {
                    let size = self.sizes[i];
                    let is_broken = size > self.space;
                    self.print_stack.push((self.indent, is_broken, *style));
                    self.writer.write_str(open)?;
                    self.space -= open.len() as isize;
                    if is_broken {
                        self.indent += 4;
                    }
                }
                Token::End { close } => {
                    let (indent, is_broken, _) = self.print_stack.pop().unwrap();
                    self.indent = indent;
                    if is_broken {
                        self.writer.write_char('\n')?;
                        for _ in 0..self.indent {
                            self.writer.write_char(' ')?;
                        }
                        self.space = self.margin - self.indent as isize;
                    }
                    self.writer.write_str(close)?;
                    self.space -= close.len() as isize;
                }
                Token::Break { len } => {
                    let (_, is_broken, style) =
                        self.print_stack.last().copied().unwrap_or((0, false, BreakStyle::Consistent));

                    let break_decision = if style == BreakStyle::Consistent {
                        is_broken
                    } else {
                        self.sizes[i] > self.space
                    };

                    if break_decision {
                        self.writer.write_char('\n')?;
                        for _ in 0..self.indent {
                            self.writer.write_char(' ')?;
                        }
                        self.space = self.margin - self.indent as isize;
                    } else {
                        for _ in 0..*len {
                            self.writer.write_char(' ')?;
                        }
                        self.space -= *len as isize;
                    }
                }
                Token::HardBreak => {
                    self.writer.write_char('\n')?;
                    for _ in 0..self.indent {
                        self.writer.write_char(' ')?;
                    }
                    self.space = self.margin - self.indent as isize;
                }
                Token::String(s) => {
                    self.writer.write_str(s)?;
                    self.space -= s.len() as isize;
                }
                Token::Comment(s) => {
                    self.writer.write_str(s)?;
                }
            }
        }
        Ok(())
    }

    /// Scans and prints the token stream to the writer.
    ///
    /// This is a convenience method that calls `scan` and then `print`.
    pub fn finish(mut self) -> fmt::Result {
        self.scan();
        self.print()
    }
}

impl PrettyPrintV2 for Comment {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.hard_break();
        match self {
            Comment::Line(s) => printer.comment(format!("//{}", s)),
            Comment::Block(s) => printer.comment(format!("/*{}*/", s)),
        }
        printer.hard_break();
        Ok(())
    }
}

impl PrettyPrintV2 for Lit {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Lit::Str(s) => printer.string(format!("\"{}\"", s)),
            Lit::Int(i) => printer.string(i.to_string()),
            Lit::Bool(b) => printer.string(b.to_string()),
        }
        Ok(())
    }
}

impl PrettyPrintV2 for BinOp {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            BinOp::Add => printer.string("+"),
            BinOp::Sub => printer.string("-"),
            BinOp::Mul => printer.string("*"),
            BinOp::Div => printer.string("/"),
        }
        Ok(())
    }
}

impl PrettyPrintV2 for ExprBinary {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.left.pretty_print_v2(printer)?;
        printer.break_();
        self.op.pretty_print_v2(printer)?;
        printer.break_();
        self.right.pretty_print_v2(printer)?;
        Ok(())
    }
}

impl PrettyPrintV2 for Expr {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Expr::Lit(lit) => lit.pretty_print_v2(printer),
            Expr::Binary(expr) => expr.pretty_print_v2(printer),
            Expr::If(expr) => expr.pretty_print_v2(printer),
            Expr::Block(expr) => expr.pretty_print_v2(printer),
            Expr::Loop(expr) => expr.pretty_print_v2(printer),
            Expr::While(expr) => expr.pretty_print_v2(printer),
            Expr::For(expr) => expr.pretty_print_v2(printer),
            Expr::Assign(expr) => expr.pretty_print_v2(printer),
            Expr::MacroCall(expr) => expr.pretty_print_v2(printer),
            Expr::Array(expr) => expr.pretty_print_v2(printer),
            Expr::Async(expr) => expr.pretty_print_v2(printer),
            Expr::Await(expr) => expr.pretty_print_v2(printer),
            Expr::Break(expr) => expr.pretty_print_v2(printer),
            Expr::Call(expr) => expr.pretty_print_v2(printer),
            Expr::Cast(expr) => expr.pretty_print_v2(printer),
            Expr::Closure(expr) => expr.pretty_print_v2(printer),
            Expr::Const(expr) => expr.pretty_print_v2(printer),
            Expr::Continue(expr) => expr.pretty_print_v2(printer),
            Expr::Field(expr) => expr.pretty_print_v2(printer),
            Expr::Index(expr) => expr.pretty_print_v2(printer),
            Expr::Match(expr) => expr.pretty_print_v2(printer),
            Expr::MethodCall(expr) => expr.pretty_print_v2(printer),
            Expr::Paren(expr) => expr.pretty_print_v2(printer),
            Expr::Range(expr) => expr.pretty_print_v2(printer),
            Expr::Reference(expr) => expr.pretty_print_v2(printer),
            Expr::Return(expr) => expr.pretty_print_v2(printer),
            Expr::Struct(expr) => expr.pretty_print_v2(printer),
            Expr::Tuple(expr) => expr.pretty_print_v2(printer),
        }
    }
}

impl PrettyPrintV2 for ExprArray {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.begin(BreakStyle::Consistent, "[");
        printer.break_();
        for (i, elem) in self.elems.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
                printer.break_();
            }
            elem.pretty_print_v2(printer)?;
        }
        printer.end("]");
        Ok(())
    }
}

impl PrettyPrintV2 for ExprAsync {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("async ");
        self.block.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for ExprAwait {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.expr.pretty_print_v2(printer)?;
        printer.string(".await");
        Ok(())
    }
}

impl PrettyPrintV2 for ExprBreak {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("break");
        Ok(())
    }
}

impl PrettyPrintV2 for ExprCall {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.func.pretty_print_v2(printer)?;
        printer.begin(BreakStyle::Consistent, "(");
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            arg.pretty_print_v2(printer)?;
        }
        printer.end(")");
        Ok(())
    }
}

impl PrettyPrintV2 for ExprCast {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.expr.pretty_print_v2(printer)?;
        printer.string(" as ");
        printer.string(&self.ty);
        Ok(())
    }
}

impl PrettyPrintV2 for ExprClosure {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("|");
        for (i, input) in self.inputs.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            printer.string(input);
        }
        printer.string("| ");
        self.body.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for ExprConst {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("const ");
        self.block.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for ExprContinue {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("continue");
        Ok(())
    }
}

impl PrettyPrintV2 for ExprField {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.expr.pretty_print_v2(printer)?;
        printer.string(".");
        printer.string(&self.member);
        Ok(())
    }
}

impl PrettyPrintV2 for ExprIndex {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.expr.pretty_print_v2(printer)?;
        printer.string("[");
        self.index.pretty_print_v2(printer)?;
        printer.string("]");
        Ok(())
    }
}

impl PrettyPrintV2 for ExprMatch {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("match ");
        self.expr.pretty_print_v2(printer)?;
        printer.string(" {");
        printer.hard_break();
        for arm in &self.arms {
            arm.pretty_print_v2(printer)?;
            printer.string(",");
            printer.hard_break();
        }
        printer.string("}");
        Ok(())
    }
}

impl PrettyPrintV2 for Arm {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(&self.pat);
        if let Some(guard) = &self.guard {
            printer.string(" if ");
            guard.pretty_print_v2(printer)?;
        }
        printer.string(" => ");
        self.body.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for ExprMethodCall {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.receiver.pretty_print_v2(printer)?;
        printer.string(".");
        printer.string(&self.method);
        printer.begin(BreakStyle::Consistent, "(");
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            arg.pretty_print_v2(printer)?;
        }
        printer.end(")");
        Ok(())
    }
}

impl PrettyPrintV2 for ExprParen {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("(");
        self.expr.pretty_print_v2(printer)?;
        printer.string(")");
        Ok(())
    }
}

impl PrettyPrintV2 for ExprRange {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        if let Some(start) = &self.start {
            start.pretty_print_v2(printer)?;
        }
        match self.limits {
            RangeLimits::HalfOpen => printer.string(".."),
            RangeLimits::Closed => printer.string("..="),
        }
        if let Some(end) = &self.end {
            end.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrintV2 for ExprRef {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("&");
        if self.is_mut {
            printer.string("mut ");
        }
        self.expr.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for ExprReturn {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("return");
        if let Some(expr) = &self.expr {
            printer.string(" ");
            expr.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrintV2 for ExprStruct {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(&self.path);
        printer.string(" {");
        printer.break_();
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                printer.string(",");
                printer.break_();
            }
            field.pretty_print_v2(printer)?;
        }
        printer.break_();
        printer.string("}");
        Ok(())
    }
}

impl PrettyPrintV2 for FieldValue {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(&self.member);
        printer.string(": ");
        self.value.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for ExprTuple {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.begin(BreakStyle::Consistent, "(");
        for (i, elem) in self.elems.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            elem.pretty_print_v2(printer)?;
        }
        printer.end(")");
        Ok(())
    }
}

impl PrettyPrintV2 for ItemFn {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("fn ");
        self.sig.pretty_print_v2(printer)?;
        printer.string(" ");
        self.block.pretty_print_v2(printer)?;
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrintV2 for Signature {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(&self.ident);
        printer.string("()");
        Ok(())
    }
}

impl PrettyPrintV2 for Block {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.begin(BreakStyle::Consistent, "{");
        let is_empty = self.stmts.is_empty()
            && self.leading_comments.is_empty()
            && self.trailing_comments.is_empty();

        if !is_empty {
            printer.hard_break();
            for comment in &self.leading_comments {
                comment.pretty_print_v2(printer)?;
            }
            for stmt in &self.stmts {
                stmt.pretty_print_v2(printer)?;
            }
            for comment in &self.trailing_comments {
                comment.pretty_print_v2(printer)?;
            }
            printer.hard_break();
        }

        printer.end("}");
        Ok(())
    }
}

impl PrettyPrintV2 for Stmt {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Stmt::Expr(expr) => {
                expr.pretty_print_v2(printer)?;
                printer.string(";");
            }
            Stmt::Let(stmt_let) => {
                stmt_let.pretty_print_v2(printer)?;
            }
        }
        Ok(())
    }
}

impl PrettyPrintV2 for StmtLet {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("let ");
        printer.string(&self.ident);
        if let Some(ty) = &self.ty {
            printer.string(": ");
            printer.string(ty);
        }
        if let Some(expr) = &self.expr {
            printer.string(" = ");
            expr.pretty_print_v2(printer)?;
        }
        printer.string(";");
        Ok(())
    }
}

impl PrettyPrintV2 for Item {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Item::Fn(item_fn) => item_fn.pretty_print_v2(printer),
            Item::Struct(item_struct) => item_struct.pretty_print_v2(printer),
            Item::Enum(item_enum) => item_enum.pretty_print_v2(printer),
            Item::Impl(item_impl) => item_impl.pretty_print_v2(printer),
            Item::Trait(item_trait) => item_trait.pretty_print_v2(printer),
        }
    }
}

impl PrettyPrintV2 for ItemStruct {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("struct ");
        printer.string(&self.ident);
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        printer.break_();
        for field in &self.fields {
            field.pretty_print_v2(printer)?;
            printer.string(",");
            printer.break_();
        }
        printer.end("}");
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrintV2 for Field {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(&self.ident);
        printer.string(": ");
        printer.string(&self.ty);
        Ok(())
    }
}

impl PrettyPrintV2 for ItemEnum {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("enum ");
        printer.string(&self.ident);
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        printer.break_();
        for variant in &self.variants {
            variant.pretty_print_v2(printer)?;
            printer.string(",");
            printer.break_();
        }
        printer.end("}");
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrintV2 for Variant {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(&self.ident);
        Ok(())
    }
}

impl PrettyPrintV2 for ItemImpl {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("impl ");
        printer.string(&self.ident);
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        printer.break_();
        for fun in &self.fns {
            fun.pretty_print_v2(printer)?;
            printer.break_();
        }
        printer.end("}");
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrintV2 for ItemTrait {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("trait ");
        printer.string(&self.ident);
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        printer.break_();
        for item in &self.items {
            item.pretty_print_v2(printer)?;
            printer.break_();
        }
        printer.end("}");
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrintV2 for TraitItem {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            TraitItem::Fn(item_fn) => item_fn.pretty_print_v2(printer),
        }
    }
}

impl PrettyPrintV2 for TraitItemFn {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for comment in &self.leading_comments {
            comment.pretty_print_v2(printer)?;
        }
        printer.string("fn ");
        self.sig.pretty_print_v2(printer)?;
        if let Some(block) = &self.block {
            printer.string(" ");
            block.pretty_print_v2(printer)?;
        } else {
            printer.string(";");
        }
        for comment in &self.trailing_comments {
            comment.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrintV2 for ExprIf {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("if ");
        self.cond.pretty_print_v2(printer)?;
        printer.string(" ");
        self.then_branch.pretty_print_v2(printer)?;
        if let Some(else_branch) = &self.else_branch {
            printer.string(" else ");
            else_branch.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrintV2 for ExprBlock {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.block.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for ExprLoop {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("loop ");
        self.body.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for ExprWhile {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("while ");
        self.cond.pretty_print_v2(printer)?;
        printer.string(" ");
        self.body.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for ExprFor {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("for ");
        printer.string(&self.pat);
        printer.string(" in ");
        self.expr.pretty_print_v2(printer)?;
        printer.string(" ");
        self.body.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for ExprAssign {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.left.pretty_print_v2(printer)?;
        printer.string(" = ");
        self.right.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for ExprMacroCall {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(&self.ident);
        printer.string("!");
        self.tokens.pretty_print_v2(printer)
    }
}

impl PrettyPrintV2 for TokenStream {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for (i, token) in self.tokens.iter().enumerate() {
            if i > 0 {
                printer.break_();
            }
            token.pretty_print_v2(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrintV2 for TokenTree {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            TokenTree::Group(group) => group.pretty_print_v2(printer),
            TokenTree::Ident(ident) => {
                printer.string(ident);
                Ok(())
            }
            TokenTree::Punct(punct) => punct.pretty_print_v2(printer),
            TokenTree::Literal(lit) => lit.pretty_print_v2(printer),
        }
    }
}

impl PrettyPrintV2 for Group {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        let (open, close) = match self.delimiter {
            Delimiter::Parenthesis => ("(", ")"),
            Delimiter::Brace => ("{", "}"),
            Delimiter::Bracket => ("[", "]"),
            Delimiter::None => ("", ""),
        };
        printer.begin(BreakStyle::Consistent, open);
        self.stream.pretty_print_v2(printer)?;
        printer.end(close);
        Ok(())
    }
}

impl PrettyPrintV2 for Punct {
    fn pretty_print_v2<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(self.ch.to_string());
        if self.spacing == Spacing::Alone {
            printer.break_();
        }
        Ok(())
    }
}
