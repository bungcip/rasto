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
//! The `PrettyPrinter` trait is implemented by all AST nodes that can be
//! pretty-printed. This trait provides a `pretty_print` method that
//! converts the AST node into a sequence of tokens for the `Printer`.

use crate::ast::item_const::ItemConst;
use crate::ast::item_extern_type::ItemExternType;
use crate::ast::item_type_alias::ItemTypeAlias;
use crate::ast::items::*;
use crate::ast::*;
use std::borrow::Cow;
use std::fmt::{self, Write};

/// The line width to aim for when formatting.
const LINE_WIDTH: isize = 100;

/// A large integer value used to represent an infinitely long line.
const INFINITY: isize = 0xffff;

/// The number of spaces to use for indentation.
const INDENT_SIZE: usize = 4;

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
}

/// A trait for types that can be pretty-printed.
pub trait PrettyPrinter {
    /// Pretty-prints the value to the given printer.
    ///
    /// # Parameters
    ///
    /// - `printer`: The `Printer` to use for formatting.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result;
}

impl<T: PrettyPrinter + ?Sized> PrettyPrinter for Box<T> {
    /// Pretty-prints the boxed value.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        (**self).pretty_print(printer)
    }
}

impl PrettyPrinter for ExprTry {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("try ");
        self.block.pretty_print(printer)
    }
}

/// Pretty-prints an AST node to a string.
///
/// This is a convenience function that creates a new `Printer`, pretty-prints
/// the given AST node, and returns the resulting string.
///
/// # Parameters
///
/// - `ast`: The AST node to pretty-print.
pub fn pretty(ast: &impl PrettyPrinter) -> String {
    let mut buf = String::new();
    let mut printer = Printer::new(&mut buf);
    ast.pretty_print(&mut printer).unwrap();
    printer.finish().unwrap();
    buf
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

    /// Scans the token stream to determine the best layout.
    ///
    /// This method implements the first pass of the pretty-printing algorithm.
    /// It calculates the size of each token group to decide whether it should
    /// be broken into multiple lines or printed on a single line.
    pub fn scan(&mut self) {
        self.sizes = vec![INFINITY; self.tokens.len()];
        self.scan_buffer_head = 0;
        self.right_total = 0;

        let mut group_has_hard_break = vec![];

        for i in 0..self.tokens.len() {
            match &self.tokens[i] {
                Token::Begin { .. } => {
                    self.scan_push(i, -self.right_total);
                    group_has_hard_break.push(false);
                }
                Token::End { .. } => {
                    let has_hard_break = group_has_hard_break.pop().unwrap_or(false);
                    loop {
                        if self.scan_buffer_head == 0 {
                            break;
                        }
                        self.scan_buffer_head -= 1;
                        let (j, offset) = self.scan_buffer[self.scan_buffer_head];
                        match self.tokens[j] {
                            Token::Begin { .. } => {
                                let len = self.right_total + offset;
                                self.sizes[j] = if has_hard_break || len > self.margin {
                                    INFINITY
                                } else {
                                    len
                                };
                                break;
                            }
                            Token::Break { .. } => {
                                let len = self.right_total + offset;
                                self.sizes[j] = if len > self.margin { INFINITY } else { len };
                            }
                            Token::HardBreak => {
                                self.sizes[j] = INFINITY;
                            }
                            _ => {}
                        }
                    }
                }
                Token::Break { .. } => {
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
                Token::HardBreak => {
                    if let Some(last) = group_has_hard_break.last_mut() {
                        *last = true;
                    }
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
                        self.indent += INDENT_SIZE;
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
                    let (_, is_broken, style) = self.print_stack.last().copied().unwrap_or((
                        0,
                        false,
                        BreakStyle::Consistent,
                    ));

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

impl PrettyPrinter for Comment {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.hard_break();
        match self {
            Comment::Line(s) => printer.string(format!("//{s}")),
            Comment::Doc(s) => printer.string(format!("///{s}")),
        }
        printer.hard_break();
        Ok(())
    }
}

impl PrettyPrinter for ItemTypeAlias {
    /// Pretty-prints the `ItemTypeAlias` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.vis.pretty_print(printer)?;
        printer.string("type ");
        self.ident.pretty_print(printer)?;
        self.generics.pretty_print(printer)?;
        printer.string(" = ");
        self.ty.pretty_print(printer)?;
        printer.string(";");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemConst {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.vis.pretty_print(printer)?;
        printer.string("const ");
        self.ident.pretty_print(printer)?;
        printer.string(": ");
        self.ty.pretty_print(printer)?;
        printer.string(" = ");
        self.expr.pretty_print(printer)?;
        printer.string(";");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemStatic {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.vis.pretty_print(printer)?;
        printer.string("static ");
        if self.is_mut {
            printer.string("mut ");
        }
        self.ident.pretty_print(printer)?;
        printer.string(": ");
        self.ty.pretty_print(printer)?;
        printer.string(" = ");
        self.expr.pretty_print(printer)?;
        printer.string(";");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemExternType {
    /// Pretty-prints the `ItemExternType` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.vis.pretty_print(printer)?;
        printer.string("extern type ");
        self.ident.pretty_print(printer)?;
        printer.string(";");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemExternBlock {
    /// Pretty-prints the `ItemExternBlock` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        if self.is_unsafe {
            printer.string("unsafe ");
        }
        printer.string("extern ");
        if let Some(abi) = &self.abi {
            printer.string(format!("\"{abi}\""));
        }
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        if !self.items.is_empty() {
            printer.hard_break();
            pp_with_breaks(&self.items, printer)?;
        }
        printer.end("}");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ExternalItem {
    /// Pretty-prints the `ExternalItem` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            ExternalItem::Static(ident, ty) => {
                printer.string("static ");
                ident.pretty_print(printer)?;
                printer.string(": ");
                ty.pretty_print(printer)?;
                printer.string(";");
            }
            ExternalItem::Fn(item_fn) => {
                printer.string("fn ");
                item_fn.sig.pretty_print(printer)?;
                printer.string(";");
            }
            ExternalItem::Macro(item_macro) => {
                item_macro.pretty_print(printer)?;
            }
            ExternalItem::Type(item_extern_type) => {
                item_extern_type.pretty_print(printer)?;
            }
        }
        Ok(())
    }
}

impl PrettyPrinter for AssociatedConst {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        printer.string("const ");
        self.ident.pretty_print(printer)?;
        printer.string(": ");
        self.ty.pretty_print(printer)?;
        if let Some(expr) = &self.expr {
            printer.string(" = ");
            expr.pretty_print(printer)?;
        }
        printer.string(";");
        Ok(())
    }
}

impl PrettyPrinter for AssociatedType {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        printer.string("type ");
        self.ident.pretty_print(printer)?;
        self.generics.pretty_print(printer)?;
        if !self.bounds.is_empty() {
            printer.string(": ");
            for (i, bound) in self.bounds.iter().enumerate() {
                if i > 0 {
                    printer.string(" + ");
                }
                bound.pretty_print(printer)?;
            }
        }
        if let Some(default) = &self.default {
            printer.string(" = ");
            default.pretty_print(printer)?;
        }
        printer.string(";");
        Ok(())
    }
}

impl PrettyPrinter for Pat {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Pat::Const(pat) => pat.pretty_print(printer),
            Pat::Ident(pat) => pat.pretty_print(printer),
            Pat::Lit(pat) => pat.pretty_print(printer),
            Pat::Macro(pat) => pat.pretty_print(printer),
            Pat::Or(pat) => pat.pretty_print(printer),
            Pat::Paren(pat) => pat.pretty_print(printer),
            Pat::Path(pat) => pat.pretty_print(printer),
            Pat::Range(pat) => pat.pretty_print(printer),
            Pat::Reference(pat) => pat.pretty_print(printer),
            Pat::Rest(pat) => pat.pretty_print(printer),
            Pat::Slice(pat) => pat.pretty_print(printer),
            Pat::Struct(pat) => pat.pretty_print(printer),
            Pat::Tuple(pat) => pat.pretty_print(printer),
            Pat::TupleStruct(pat) => pat.pretty_print(printer),
            Pat::Type(pat) => pat.pretty_print(printer),
            Pat::Wild(pat) => pat.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for PatConst {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("const ");
        self.expr.pretty_print(printer)
    }
}

impl PrettyPrinter for PatIdent {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        if self.is_mut {
            printer.string("mut ");
        }
        self.ident.pretty_print(printer)?;
        Ok(())
    }
}

impl PrettyPrinter for PatLit {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.lit.pretty_print(printer)
    }
}

impl PrettyPrinter for PatMacro {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.mac.pretty_print(printer)
    }
}

impl PrettyPrinter for PatOr {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for (i, pat) in self.pats.iter().enumerate() {
            if i > 0 {
                printer.string(" | ");
            }
            pat.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for PatParen {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("(");
        self.pat.pretty_print(printer)?;
        printer.string(")");
        Ok(())
    }
}

impl PrettyPrinter for PatPath {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.path.pretty_print(printer)
    }
}

impl PrettyPrinter for PatRange {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        if let Some(start) = &self.start {
            start.pretty_print(printer)?;
        }
        match self.limits {
            RangeLimits::HalfOpen => printer.string(".."),
            RangeLimits::Closed => printer.string("..="),
        }
        if let Some(end) = &self.end {
            end.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for PatReference {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("&");
        if self.is_mut {
            printer.string("mut ");
        }
        self.pat.pretty_print(printer)
    }
}

impl PrettyPrinter for PatRest {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("..");
        Ok(())
    }
}

impl PrettyPrinter for PatSlice {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.begin(BreakStyle::Consistent, "[");
        for (i, pat) in self.pats.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            pat.pretty_print(printer)?;
        }
        printer.end("]");
        Ok(())
    }
}

impl PrettyPrinter for PatStruct {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.path.pretty_print(printer)?;
        printer.begin(BreakStyle::Consistent, " {");
        printer.break_();
        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                printer.string(",");
                printer.break_();
            }
            field.pretty_print(printer)?;
        }
        if self.has_rest {
            if !self.fields.is_empty() {
                printer.string(",");
                printer.break_();
            }
            printer.string("..");
        }
        printer.break_();
        printer.end("}");
        Ok(())
    }
}

impl PrettyPrinter for FieldPat {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.member.pretty_print(printer)?;
        printer.string(": ");
        self.pat.pretty_print(printer)
    }
}

impl PrettyPrinter for PatTuple {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.begin(BreakStyle::Consistent, "(");
        for (i, pat) in self.pats.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            pat.pretty_print(printer)?;
        }
        printer.end(")");
        Ok(())
    }
}

impl PrettyPrinter for PatTupleStruct {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.path.pretty_print(printer)?;
        printer.begin(BreakStyle::Consistent, "(");
        for (i, pat) in self.pats.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            pat.pretty_print(printer)?;
        }
        printer.end(")");
        Ok(())
    }
}

impl PrettyPrinter for PatType {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.pat.pretty_print(printer)?;
        printer.string(": ");
        self.ty.pretty_print(printer)
    }
}

impl PrettyPrinter for PatWild {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("_");
        Ok(())
    }
}

impl PrettyPrinter for UnOp {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            UnOp::Not => printer.string("!"),
            UnOp::Neg => printer.string("-"),
        }
        Ok(())
    }
}

impl PrettyPrinter for Path {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for (i, segment) in self.segments.iter().enumerate() {
            if i > 0 {
                printer.string("::");
            }
            segment.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for PathSegment {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.ident.pretty_print(printer)?;
        if let Some(args) = &self.args {
            args.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for Lit {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Lit::Str(lit) => lit.pretty_print(printer),
            Lit::ByteStr(lit) => lit.pretty_print(printer),
            Lit::CStr(lit) => lit.pretty_print(printer),
            Lit::Byte(lit) => lit.pretty_print(printer),
            Lit::Char(lit) => lit.pretty_print(printer),
            Lit::Int(lit) => lit.pretty_print(printer),
            Lit::Float(lit) => lit.pretty_print(printer),
            Lit::Bool(lit) => lit.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for LitStr {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(format!("\"{}\"", self.value));
        Ok(())
    }
}

impl PrettyPrinter for LitByteStr {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(format!("b\"{}\"", String::from_utf8_lossy(&self.value)));
        Ok(())
    }
}

impl PrettyPrinter for LitCStr {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(format!("c\"{}\"", String::from_utf8_lossy(&self.value)));
        Ok(())
    }
}

impl PrettyPrinter for LitByte {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(format!("b'{}'", self.value as char));
        Ok(())
    }
}

impl PrettyPrinter for LitChar {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(format!("'{}'", self.value));
        Ok(())
    }
}

impl PrettyPrinter for LitInt {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(self.value.to_string());
        if let Some(suffix) = &self.suffix {
            let s = match suffix {
                IntSuffix::U8 => "u8",
                IntSuffix::I8 => "i8",
                IntSuffix::U16 => "u16",
                IntSuffix::I16 => "i16",
                IntSuffix::U32 => "u32",
                IntSuffix::I32 => "i32",
                IntSuffix::U64 => "u64",
                IntSuffix::I64 => "i64",
                IntSuffix::U128 => "u128",
                IntSuffix::I128 => "i128",
                IntSuffix::Usize => "usize",
                IntSuffix::Isize => "isize",
            };
            printer.string(s);
        }
        Ok(())
    }
}

impl PrettyPrinter for LitFloat {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(&self.value);
        if let Some(suffix) = &self.suffix {
            let s = match suffix {
                FloatSuffix::F32 => "f32",
                FloatSuffix::F64 => "f64",
            };
            printer.string(s);
        }
        Ok(())
    }
}

impl PrettyPrinter for LitBool {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(self.value.to_string());
        Ok(())
    }
}

impl PrettyPrinter for BinOp {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            BinOp::Add => printer.string("+"),
            BinOp::Sub => printer.string("-"),
            BinOp::Mul => printer.string("*"),
            BinOp::Div => printer.string("/"),
            BinOp::Eq => printer.string("=="),
            BinOp::Lt => printer.string("<"),
            BinOp::Le => printer.string("<="),
            BinOp::Ne => printer.string("!="),
            BinOp::Ge => printer.string(">="),
            BinOp::Gt => printer.string(">"),
        }
        Ok(())
    }
}

impl PrettyPrinter for ExprBinary {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        let precedence = self.op.precedence();
        printer.begin(BreakStyle::Inconsistent, "");
        pretty_print_expr(&self.left, printer, precedence, true)?;
        printer.break_();
        self.op.pretty_print(printer)?;
        printer.string(" ");
        pretty_print_expr(&self.right, printer, precedence, false)?;
        printer.end("");
        Ok(())
    }
}

impl PrettyPrinter for ExprUnary {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.op.pretty_print(printer)?;
        self.expr.pretty_print(printer)?;
        Ok(())
    }
}

impl PrettyPrinter for Expr {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pretty_print_expr(self, printer, 0, false)
    }
}

fn pretty_print_expr<'a>(
    expr: &'a Expr,
    printer: &mut Printer<'a>,
    parent_precedence: u8,
    is_left: bool,
) -> fmt::Result {
    match expr {
        Expr::Binary(binary) => {
            let precedence = binary.op.precedence();
            let needs_paren = if is_left {
                precedence < parent_precedence
            } else {
                precedence <= parent_precedence
            };

            if needs_paren {
                printer.string("(");
                binary.pretty_print(printer)?;
                printer.string(")");
            } else {
                binary.pretty_print(printer)?;
            }
        }
        Expr::Lit(lit) => lit.pretty_print(printer)?,
        Expr::If(expr) => expr.pretty_print(printer)?,
        Expr::Block(expr) => expr.pretty_print(printer)?,
        Expr::Loop(expr) => expr.pretty_print(printer)?,
        Expr::While(expr) => expr.pretty_print(printer)?,
        Expr::For(expr) => expr.pretty_print(printer)?,
        Expr::Assign(expr) => expr.pretty_print(printer)?,
        Expr::MacroCall(expr) => expr.pretty_print(printer)?,
        Expr::Array(expr) => expr.pretty_print(printer)?,
        Expr::Async(expr) => expr.pretty_print(printer)?,
        Expr::Await(expr) => expr.pretty_print(printer)?,
        Expr::Break(expr) => expr.pretty_print(printer)?,
        Expr::Call(expr) => expr.pretty_print(printer)?,
        Expr::Cast(expr) => expr.pretty_print(printer)?,
        Expr::Closure(expr) => expr.pretty_print(printer)?,
        Expr::Const(expr) => expr.pretty_print(printer)?,
        Expr::Continue(expr) => expr.pretty_print(printer)?,
        Expr::Field(expr) => expr.pretty_print(printer)?,
        Expr::Gen(expr) => expr.pretty_print(printer)?,
        Expr::Index(expr) => expr.pretty_print(printer)?,
        Expr::Match(expr) => expr.pretty_print(printer)?,
        Expr::MethodCall(expr) => expr.pretty_print(printer)?,
        Expr::Paren(expr) => expr.pretty_print(printer)?,
        Expr::Path(expr) => expr.pretty_print(printer)?,
        Expr::Range(expr) => expr.pretty_print(printer)?,
        Expr::Reference(expr) => expr.pretty_print(printer)?,
        Expr::RawRef(expr) => expr.pretty_print(printer)?,
        Expr::Return(expr) => expr.pretty_print(printer)?,
        Expr::Struct(expr) => expr.pretty_print(printer)?,
        Expr::Try(expr) => expr.pretty_print(printer)?,
        Expr::Tuple(expr) => expr.pretty_print(printer)?,
        Expr::Infer(expr) => expr.pretty_print(printer)?,
        Expr::Unary(expr) => expr.pretty_print(printer)?,
    }
    Ok(())
}

impl PrettyPrinter for ExprInfer {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("_");
        Ok(())
    }
}

impl PrettyPrinter for ExprArray {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.begin(BreakStyle::Consistent, "[");
        printer.break_();
        for (i, elem) in self.elems.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
                printer.break_();
            }
            elem.pretty_print(printer)?;
        }
        printer.end("]");
        Ok(())
    }
}

impl PrettyPrinter for ExprAsync {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("async ");
        self.block.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprGen {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("gen ");
        self.block.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprAwait {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.expr.pretty_print(printer)?;
        printer.string(".await");
        Ok(())
    }
}

impl PrettyPrinter for ExprBreak {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("break");
        Ok(())
    }
}

impl PrettyPrinter for ExprCall {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.func.pretty_print(printer)?;
        printer.begin(BreakStyle::Consistent, "(");
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            arg.pretty_print(printer)?;
        }
        printer.end(")");
        Ok(())
    }
}

impl PrettyPrinter for ExprCast {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.expr.pretty_print(printer)?;
        printer.string(" as ");
        self.ty.pretty_print(printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ExprClosure {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("|");
        for (i, input) in self.inputs.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            input.pretty_print(printer)?;
        }
        printer.string("| ");
        self.body.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprConst {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("const ");
        self.block.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprContinue {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("continue");
        Ok(())
    }
}

impl PrettyPrinter for ExprField {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.expr.pretty_print(printer)?;
        printer.string(".");
        self.member.pretty_print(printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ExprIndex {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.expr.pretty_print(printer)?;
        printer.string("[");
        self.index.pretty_print(printer)?;
        printer.string("]");
        Ok(())
    }
}

impl PrettyPrinter for ExprMatch {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("match ");
        self.expr.pretty_print(printer)?;
        printer.begin(BreakStyle::Consistent, " {");
        printer.hard_break();
        let num_arms = self.arms.len();
        for (i, arm) in self.arms.iter().enumerate() {
            arm.pretty_print(printer)?;
            printer.string(",");
            if i < num_arms - 1 {
                printer.hard_break();
            }
        }
        printer.end("}");
        Ok(())
    }
}

impl PrettyPrinter for Arm {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.pat.pretty_print(printer)?;
        if let Some(guard) = &self.guard {
            printer.string(" if ");
            guard.pretty_print(printer)?;
        }
        printer.string(" => ");
        self.body.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprMethodCall {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.receiver.pretty_print(printer)?;
        printer.string(".");
        self.method.pretty_print(printer)?;
        printer.begin(BreakStyle::Consistent, "(");
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            arg.pretty_print(printer)?;
        }
        printer.end(")");
        Ok(())
    }
}

impl PrettyPrinter for ExprParen {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("(");
        self.expr.pretty_print(printer)?;
        printer.string(")");
        Ok(())
    }
}

impl PrettyPrinter for ExprPath {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.path.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprRange {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        if let Some(start) = &self.start {
            start.pretty_print(printer)?;
        }
        match self.limits {
            RangeLimits::HalfOpen => printer.string(".."),
            RangeLimits::Closed => printer.string("..="),
        }
        if let Some(end) = &self.end {
            end.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for ExprRef {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("&");
        if self.is_mut {
            printer.string("mut ");
        }
        self.expr.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprRawRef {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("&raw ");
        if self.is_mut {
            printer.string("mut ");
        } else {
            printer.string("const ");
        }
        self.expr.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprReturn {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("return");
        if let Some(expr) = &self.expr {
            printer.string(" ");
            expr.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for ExprStruct {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.path.pretty_print(printer)?;
        if !self.fields.is_empty() {
            printer.begin(BreakStyle::Consistent, " {");
            printer.break_();
            for (i, field) in self.fields.iter().enumerate() {
                if i > 0 {
                    printer.string(",");
                    printer.break_();
                }
                field.pretty_print(printer)?;
            }
            printer.break_();
            printer.end("}");
        }
        Ok(())
    }
}

impl PrettyPrinter for FieldValue {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.member.pretty_print(printer)?;
        printer.string(": ");
        self.value.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprTuple {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.begin(BreakStyle::Consistent, "(");
        for (i, elem) in self.elems.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            elem.pretty_print(printer)?;
        }
        printer.end(")");
        Ok(())
    }
}

impl PrettyPrinter for ItemFn {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.vis.pretty_print(printer)?;
        printer.string("fn ");
        self.sig.pretty_print(printer)?;
        printer.string(" ");
        self.block.pretty_print(printer)?;
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for Signature {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        if self.is_const {
            printer.string("const ");
        }
        if self.is_async {
            printer.string("async ");
        }
        if self.is_unsafe {
            printer.string("unsafe ");
        }
        if let Some(abi) = &self.abi {
            printer.string("extern ");
            abi.pretty_print(printer)?;
            printer.string(" ");
        }
        self.ident.pretty_print(printer)?;
        self.generics.pretty_print(printer)?;
        printer.begin(BreakStyle::Consistent, "(");
        for (i, input) in self.inputs.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            input.pretty_print(printer)?;
        }
        if self.is_variadic {
            if !self.inputs.is_empty() {
                printer.string(", ");
            }
            printer.string("...");
        }
        printer.end(")");
        if let Some(output) = &self.output {
            printer.string(" -> ");
            output.pretty_print(printer)?;
        }
        if let Some(where_clause) = &self.where_clause {
            where_clause.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for Block {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.begin(BreakStyle::Consistent, "{");

        let is_empty = self.stmts.is_empty() && self.md.is_none();

        if !is_empty {
            printer.hard_break();
            pp_begin(&self.md, printer)?;

            let num_stmts = self.stmts.len();
            for (i, stmt) in self.stmts.iter().enumerate() {
                stmt.pretty_print(printer)?;

                let is_last = i == num_stmts - 1;

                if matches!(stmt, Stmt::Expr(_)) && (!is_last || self.has_trailing_semicolon) {
                    printer.string(";");
                }

                if !is_last {
                    printer.hard_break();
                }
            }

            pp_end(&self.md, printer)?;
        }

        printer.end("}");
        Ok(())
    }
}

impl PrettyPrinter for Stmt {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Stmt::Local(local) => {
                local.pretty_print(printer)?;
            }
            Stmt::Item(item) => {
                item.pretty_print(printer)?;
            }
            Stmt::Expr(expr) => {
                expr.pretty_print(printer)?;
            }
        }
        Ok(())
    }
}

impl PrettyPrinter for Local {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("let ");
        self.pat.pretty_print(printer)?;
        if let Some(ty) = &self.ty {
            printer.string(": ");
            ty.pretty_print(printer)?;
        }
        if let Some(expr) = &self.expr {
            printer.string(" = ");
            expr.pretty_print(printer)?;
        }
        if let Some(else_block) = &self.else_block {
            printer.string(" else ");
            else_block.pretty_print(printer)?;
        } else {
            printer.string(";");
        }
        Ok(())
    }
}

impl PrettyPrinter for Item {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Item::Fn(item_fn) => item_fn.pretty_print(printer),
            Item::Const(item_const) => item_const.pretty_print(printer),
            Item::Struct(item_struct) => item_struct.pretty_print(printer),
            Item::Static(item_static) => item_static.pretty_print(printer),
            Item::Enum(item_enum) => item_enum.pretty_print(printer),
            Item::Impl(item_impl) => item_impl.pretty_print(printer),
            Item::Trait(item_trait) => item_trait.pretty_print(printer),
            Item::ExternCrate(item_extern_crate) => item_extern_crate.pretty_print(printer),
            Item::ForeignMod(item_foreign_mod) => item_foreign_mod.pretty_print(printer),
            Item::Macro(item_macro) => item_macro.pretty_print(printer),
            Item::Mod(item_mod) => item_mod.pretty_print(printer),
            Item::TraitAlias(item_trait_alias) => item_trait_alias.pretty_print(printer),
            Item::TypeAlias(item_type_alias) => item_type_alias.pretty_print(printer),
            Item::Union(item_union) => item_union.pretty_print(printer),
            Item::Use(item_use) => item_use.pretty_print(printer),
            Item::Asm(item_asm) => item_asm.pretty_print(printer),
            Item::ExternBlock(item_extern_block) => item_extern_block.pretty_print(printer),
            Item::ExternType(item_extern_type) => item_extern_type.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for ItemAsm {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("asm!(");
        printer.begin(BreakStyle::Consistent, "");
        for (i, lit) in self.template.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            lit.pretty_print(printer)?;
        }

        if !self.operands.is_empty() || self.options.is_some() {
            printer.string(",");
            printer.break_();
        }

        for (i, operand) in self.operands.iter().enumerate() {
            if i > 0 {
                printer.string(",");
                printer.break_();
            }
            operand.pretty_print(printer)?;
        }

        if !self.operands.is_empty() && self.options.is_some() {
            printer.string(",");
            printer.break_();
        }

        if let Some(options) = &self.options {
            options.pretty_print(printer)?;
        }

        printer.end(")");
        Ok(())
    }
}

impl PrettyPrinter for AsmOperand {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            AsmOperand::Reg(reg) => reg.pretty_print(printer),
            AsmOperand::Sym(path) => {
                printer.string("sym ");
                path.pretty_print(printer)
            }
            AsmOperand::Const(expr) => {
                printer.string("const ");
                expr.pretty_print(printer)
            }
            AsmOperand::ClobberAbi(clobber) => clobber.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for RegOperand {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.direction.pretty_print(printer)?;
        printer.string("(");
        self.reg.pretty_print(printer)?;
        printer.string(") ");
        self.expr.pretty_print(printer)?;
        if let Some(out_expr) = &self.out_expr {
            printer.string(" => ");
            out_expr.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for AsmDirection {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            AsmDirection::In => printer.string("in"),
            AsmDirection::Out => printer.string("out"),
            AsmDirection::LateOut => printer.string("lateout"),
            AsmDirection::InOut => printer.string("inout"),
            AsmDirection::InLateOut => printer.string("inlateout"),
        }
        Ok(())
    }
}

impl PrettyPrinter for RegSpec {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            RegSpec::Class(class) => printer.string(class),
            RegSpec::Explicit(reg) => reg.pretty_print(printer)?,
        }
        Ok(())
    }
}

impl PrettyPrinter for AsmOptions {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("options(");
        for (i, option) in self.options.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            option.pretty_print(printer)?;
        }
        printer.string(")");
        Ok(())
    }
}

impl PrettyPrinter for AsmOption {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            AsmOption::Pure => printer.string("pure"),
            AsmOption::Nomem => printer.string("nomem"),
            AsmOption::ReadOnly => printer.string("readonly"),
            AsmOption::PreservesFlags => printer.string("preserves_flags"),
            AsmOption::NoReturn => printer.string("noreturn"),
            AsmOption::NoStack => printer.string("nostack"),
            AsmOption::AttSyntax => printer.string("att_syntax"),
            AsmOption::Raw => printer.string("raw"),
        }
        Ok(())
    }
}

impl PrettyPrinter for ClobberAbi {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("clobber_abi(");
        for (i, abi) in self.abis.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            abi.pretty_print(printer)?;
        }
        printer.string(")");
        Ok(())
    }
}

fn pp_separated_with_trailing<'a, T: PrettyPrinter>(
    items: &'a [T],
    separator: &'a str,
    printer: &mut Printer<'a>,
) -> fmt::Result {
    let num_items = items.len();
    for (i, item) in items.iter().enumerate() {
        item.pretty_print(printer)?;
        printer.string(separator);
        if i < num_items - 1 {
            printer.hard_break();
        }
    }
    Ok(())
}

fn pp_with_breaks<'a, T: PrettyPrinter>(items: &'a [T], printer: &mut Printer<'a>) -> fmt::Result {
    let num_items = items.len();
    for (i, item) in items.iter().enumerate() {
        item.pretty_print(printer)?;
        if i < num_items - 1 {
            printer.hard_break();
        }
    }
    Ok(())
}

impl PrettyPrinter for File {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        pp_with_breaks(&self.items, printer)?;
        pp_end(&self.md, printer)
    }
}

impl PrettyPrinter for ItemStruct {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.vis.pretty_print(printer)?;
        printer.string("struct ");
        self.ident.pretty_print(printer)?;
        self.generics.pretty_print(printer)?;
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        if !self.fields.is_empty() {
            printer.hard_break();
            pp_separated_with_trailing(&self.fields, ",", printer)?;
        }
        printer.end("}");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for Field {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.ident.pretty_print(printer)?;
        printer.string(": ");
        self.ty.pretty_print(printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemEnum {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.vis.pretty_print(printer)?;
        printer.string("enum ");
        self.ident.pretty_print(printer)?;
        self.generics.pretty_print(printer)?;
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        if !self.variants.is_empty() {
            printer.hard_break();
            pp_separated_with_trailing(&self.variants, ",", printer)?;
        }
        printer.end("}");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for Variant {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.ident.pretty_print(printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ImplItem {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            ImplItem::Fn(item_fn) => item_fn.pretty_print(printer),
            ImplItem::Type(associated_type) => associated_type.pretty_print(printer),
            ImplItem::Const(associated_const) => associated_const.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for ItemImpl {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        if self.is_unsafe {
            printer.string("unsafe ");
        }
        printer.string("impl");
        self.generics.pretty_print(printer)?;
        printer.string(" ");
        if self.is_negative {
            printer.string("!");
        }
        if let Some(trait_) = &self.trait_ {
            trait_.pretty_print(printer)?;
            printer.string(" for ");
        }
        self.ty.pretty_print(printer)?;
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        if !self.items.is_empty() {
            printer.hard_break();
            pp_with_breaks(&self.items, printer)?;
        }
        printer.end("}");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemTrait {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.vis.pretty_print(printer)?;
        printer.string("trait ");
        self.ident.pretty_print(printer)?;
        self.generics.pretty_print(printer)?;
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");

        if !self.associated_types.is_empty() {
            printer.hard_break();
            pp_with_breaks(&self.associated_types, printer)?;
        }

        if !self.items.is_empty() {
            printer.hard_break();
            pp_with_breaks(&self.items, printer)?;
        }

        printer.end("}");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for TraitItem {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            TraitItem::Fn(item_fn) => item_fn.pretty_print(printer),
            TraitItem::Const(associated_const) => associated_const.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for TraitItemFn {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        printer.string("fn ");
        self.sig.pretty_print(printer)?;
        if let Some(block) = &self.block {
            printer.string(" ");
            block.pretty_print(printer)?;
        } else {
            printer.string(";");
        }
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ExprIf {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("if ");
        self.cond.pretty_print(printer)?;
        printer.string(" ");
        self.then_branch.pretty_print(printer)?;
        if let Some(else_branch) = &self.else_branch {
            printer.string(" else ");
            else_branch.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for ExprBlock {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.block.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprLoop {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("loop ");
        self.body.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprWhile {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("while ");
        self.cond.pretty_print(printer)?;
        printer.string(" ");
        self.body.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprFor {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("for ");
        self.pat.pretty_print(printer)?;
        printer.string(" in ");
        self.expr.pretty_print(printer)?;
        printer.string(" ");
        self.body.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprAssign {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.left.pretty_print(printer)?;
        printer.string(" = ");
        self.right.pretty_print(printer)
    }
}

impl PrettyPrinter for ExprMacroCall {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.path.pretty_print(printer)?;
        printer.string("!");

        let (open, close) = match self.delimiter {
            Delimiter::Parenthesis => ("(", ")"),
            Delimiter::Brace => ("{", "}"),
            Delimiter::Bracket => ("[", "]"),
            Delimiter::None => ("", ""),
        };

        printer.begin(BreakStyle::Consistent, open);
        self.tokens.pretty_print(printer)?;
        printer.end(close);

        Ok(())
    }
}

impl PrettyPrinter for TokenStream {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        for (i, token) in self.tokens.iter().enumerate() {
            if i > 0 {
                printer.break_();
            }
            token.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for TokenTree {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            TokenTree::Group(group) => group.pretty_print(printer),
            TokenTree::Ident(ident) => {
                printer.string(ident);
                Ok(())
            }
            TokenTree::Punct(punct) => punct.pretty_print(printer),
            TokenTree::Literal(lit) => lit.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for Group {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        let (open, close) = match self.delimiter {
            Delimiter::Parenthesis => ("(", ")"),
            Delimiter::Brace => ("{", "}"),
            Delimiter::Bracket => ("[", "]"),
            Delimiter::None => ("", ""),
        };
        printer.begin(BreakStyle::Consistent, open);
        self.stream.pretty_print(printer)?;
        printer.end(close);
        Ok(())
    }
}

impl PrettyPrinter for Punct {
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string(self.ch.to_string());
        if self.spacing == Spacing::Alone {
            printer.break_();
        }
        Ok(())
    }
}

impl PrettyPrinter for Attribute {
    /// Pretty-prints the `Attribute` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Attribute::Inner(meta) => {
                printer.string("#![");
                meta.pretty_print(printer)?;
                printer.string("]");
            }
            Attribute::Outer(meta) => {
                printer.string("#[");
                meta.pretty_print(printer)?;
                printer.string("]");
            }
        }
        Ok(())
    }
}

impl PrettyPrinter for Meta {
    /// Pretty-prints the `Meta` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Meta::List(list) => list.pretty_print(printer),
            Meta::Path(path) => path.pretty_print(printer),
            Meta::NameValue(name_value) => name_value.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for MetaList {
    /// Pretty-prints the `MetaList` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.path.pretty_print(printer)?;
        printer.string("(");
        for (i, meta) in self.metas.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            meta.pretty_print(printer)?;
        }
        printer.string(")");
        Ok(())
    }
}

impl PrettyPrinter for MetaNameValue {
    /// Pretty-prints the `MetaNameValue` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.path.pretty_print(printer)?;
        printer.string(" = ");
        self.value.pretty_print(printer)
    }
}

impl PrettyPrinter for GenericArgs {
    /// Pretty-prints the `GenericArgs` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        if !self.args.is_empty() {
            printer.string("<");
            for (i, arg) in self.args.iter().enumerate() {
                if i > 0 {
                    printer.string(", ");
                }
                arg.pretty_print(printer)?;
            }
            printer.string(">");
        }
        Ok(())
    }
}

impl PrettyPrinter for GenericArg {
    /// Pretty-prints the `GenericArg` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            GenericArg::Lifetime(lt) => {
                printer.string("'");
                lt.pretty_print(printer)
            }
            GenericArg::Type(t) => t.pretty_print(printer),
            GenericArg::Const(c) => c.pretty_print(printer),
        }
    }
}

impl PrettyPrinter for ItemExternCrate {
    /// Pretty-prints the `ItemExternCrate` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        printer.string("extern crate ");
        self.ident.pretty_print(printer)?;
        printer.string(";");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemForeignMod {
    /// Pretty-prints the `ItemForeignMod` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        printer.string("extern ");
        printer.string(format!("\"{}\"", self.abi));
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        if !self.items.is_empty() {
            printer.hard_break();
            pp_with_breaks(&self.items, printer)?;
        }
        printer.end("}");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemMacro {
    /// Pretty-prints the `ItemMacro` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.expr.pretty_print(printer)?;
        printer.string(";");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemMod {
    /// Pretty-prints the `ItemMod` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.vis.pretty_print(printer)?;
        printer.string("mod ");
        self.ident.pretty_print(printer)?;
        if let Some(content) = &self.content {
            printer.string(" ");
            printer.begin(BreakStyle::Consistent, "{");
            if !content.is_empty() {
                printer.hard_break();
                pp_with_breaks(content, printer)?;
            }
            printer.end("}");
        } else {
            printer.string(";");
        }
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemTraitAlias {
    /// Pretty-prints the `ItemTraitAlias` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        printer.string("trait ");
        self.ident.pretty_print(printer)?;
        printer.string(" = ");
        for (i, bound) in self.bounds.iter().enumerate() {
            if i > 0 {
                printer.string(" + ");
            }
            printer.string(bound);
        }
        printer.string(";");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemUnion {
    /// Pretty-prints the `ItemUnion` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.vis.pretty_print(printer)?;
        printer.string("union ");
        self.ident.pretty_print(printer)?;
        self.generics.pretty_print(printer)?;
        printer.string(" ");
        printer.begin(BreakStyle::Consistent, "{");
        if !self.fields.is_empty() {
            printer.hard_break();
            pp_separated_with_trailing(&self.fields, ",", printer)?;
        }
        printer.end("}");

        pp_end(&self.md, printer)?;
        Ok(())
    }
}

impl PrettyPrinter for ItemUse {
    /// Pretty-prints the `ItemUse` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        pp_begin(&self.md, printer)?;
        self.vis.pretty_print(printer)?;
        printer.string("use ");
        printer.string(&self.path);
        printer.string(";");
        pp_end(&self.md, printer)?;
        Ok(())
    }
}

/// Pretty-prints the leading metadata of an AST node.
///
/// This includes attributes and comments.
pub fn pp_begin<'a>(md: &'a Option<Box<Md>>, printer: &mut Printer<'a>) -> fmt::Result {
    if let Some(md) = &md {
        for attr in &md.attrs {
            attr.pretty_print(printer)?;
            printer.hard_break();
        }
        for comment in &md.comments {
            comment.pretty_print(printer)?;
        }
    }
    Ok(())
}

/// Pretty-prints the trailing metadata of an AST node.
///
/// This includes trailing comments.
pub fn pp_end<'a>(md: &'a Option<Box<Md>>, printer: &mut Printer<'a>) -> fmt::Result {
    if let Some(md) = &md {
        for comment in &md.trailing_comments {
            comment.pretty_print(printer)?;
        }
    }
    Ok(())
}

impl PrettyPrinter for Type {
    /// Pretty-prints the `Type` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        match self {
            Type::Array(array) => array.pretty_print(printer),
            Type::BareFn(bare_fn) => bare_fn.pretty_print(printer),
            Type::Group(group) => group.pretty_print(printer),
            Type::ImplTrait => {
                printer.string("impl Trait");
                Ok(())
            }
            Type::Infer => {
                printer.string("_");
                Ok(())
            }
            Type::Macro(mac) => mac.pretty_print(printer),
            Type::Never => {
                printer.string("!");
                Ok(())
            }
            Type::Paren(paren) => {
                printer.string("(");
                paren.pretty_print(printer)?;
                printer.string(")");
                Ok(())
            }
            Type::Path(path) => path.pretty_print(printer),
            Type::Ptr(ptr) => ptr.pretty_print(printer),
            Type::Reference(reference) => reference.pretty_print(printer),
            Type::Slice(slice) => {
                printer.string("[");
                slice.pretty_print(printer)?;
                printer.string("]");
                Ok(())
            }
            Type::TraitObject => {
                printer.string("dyn Trait");
                Ok(())
            }
            Type::Tuple(tuple) => {
                printer.string("(");
                for (i, ty) in tuple.iter().enumerate() {
                    if i > 0 {
                        printer.string(", ");
                    }
                    ty.pretty_print(printer)?;
                }
                if tuple.len() == 1 {
                    printer.string(",");
                }
                printer.string(")");
                Ok(())
            }
        }
    }
}

impl PrettyPrinter for TypeArray {
    /// Pretty-prints the `TypeArray` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("[");
        self.elem.pretty_print(printer)?;
        printer.string("; ");
        self.len.pretty_print(printer)?;
        printer.string("]");
        Ok(())
    }
}

impl PrettyPrinter for TypeBareFn {
    /// Pretty-prints the `TypeBareFn` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("fn(");
        for (i, ty) in self.inputs.iter().enumerate() {
            if i > 0 {
                printer.string(", ");
            }
            ty.pretty_print(printer)?;
        }
        printer.string(")");
        if let Some(output) = &self.output {
            printer.string(" -> ");
            output.pretty_print(printer)?;
        }
        Ok(())
    }
}

impl PrettyPrinter for TypePath {
    /// Pretty-prints the `TypePath` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        self.path.pretty_print(printer)
    }
}

impl PrettyPrinter for TypePtr {
    /// Pretty-prints the `TypePtr` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("*");
        if self.mutable {
            printer.string("mut ");
        } else {
            printer.string("const ");
        }
        self.elem.pretty_print(printer)
    }
}

impl PrettyPrinter for TypeReference {
    /// Pretty-prints the `TypeReference` to the given printer.
    fn pretty_print<'a>(&'a self, printer: &mut Printer<'a>) -> fmt::Result {
        printer.string("&");
        if let Some(lifetime) = &self.lifetime {
            lifetime.pretty_print(printer)?;
            printer.string(" ");
        }
        if self.mutable {
            printer.string("mut ");
        }
        self.elem.pretty_print(printer)
    }
}
