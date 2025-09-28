//! Defines the AST node for an `asm!` expression.

use crate::ast::{Expr, LitStr, Path};
use crate::pretty_printer::PrettyPrinter;
use thin_vec::ThinVec;

ast_item! {
    /// Represents an inline assembly block.
    ///
    /// This is used for embedding assembly code directly into Rust functions.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::arch::asm;
    /// unsafe {
///     let x: u64 = 3;
///     let y: u64;
    ///     asm!(
    ///         "mov {0}, {1}",
///         out(reg) y,
///         in(reg) x,
    ///     );
    /// }
    /// ```
    pub struct ItemAsm without vis, ident, and md {
        /// A collection of string literals that make up the assembly code template.
        pub template: ThinVec<LitStr>,
        /// The list of input, output, and other operands for the assembly code.
        pub operands: ThinVec<AsmOperand>,
        /// A set of options that control the behavior of the assembly block,
        /// such as `pure`, `nomem`, or `att_syntax`.
        pub options: Option<AsmOptions>,
    }
}

/// An operand for an `asm!` expression.
#[derive(Debug, Clone, PartialEq)]
pub enum AsmOperand {
    /// A register operand.
    Reg(RegOperand),
    /// A `sym` operand.
    Sym(Path),
    /// A `const` operand.
    Const(Expr),
    /// A `clobber_abi` operand.
    ClobberAbi(ClobberAbi),
}

/// A register operand for an `asm!` expression.
#[derive(Debug, Clone, PartialEq)]
pub struct RegOperand {
    /// The name of the operand, if specified (e.g., `bytes` in `bytes = out(reg) ...`).
    pub name: Option<String>,
    /// The `in`, `out`, `inout`, `lateout`, or `inlateout` keyword.
    pub direction: AsmDirection,
    /// The register specifier.
    pub reg: RegSpec,
    /// The expression providing the value for the register.
    pub expr: Expr,
    /// The output expression for `inout` operands.
    pub out_expr: Option<Expr>,
}

/// The direction of a register operand.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsmDirection {
    /// `in`
    In,
    /// `out`
    Out,
    /// `lateout`
    LateOut,
    /// `inout`
    InOut,
    /// `inlateout`
    InLateOut,
}

/// The register specifier for a register operand.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegSpec {
    /// A register class, e.g., `reg`.
    Class(String),
    /// An explicit register, e.g., `"eax"`.
    Explicit(LitStr),
}

/// The options for an `asm!` expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AsmOptions {
    /// The list of options.
    pub options: ThinVec<AsmOption>,
}

/// An option for an `asm!` expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsmOption {
    /// `pure`
    Pure,
    /// `nomem`
    Nomem,
    /// `readonly`
    ReadOnly,
    /// `preserves_flags`
    PreservesFlags,
    /// `noreturn`
    NoReturn,
    /// `nostack`
    NoStack,
    /// `att_syntax`
    AttSyntax,
    /// `raw`
    Raw,
}

/// A `clobber_abi` operand for an `asm!` expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClobberAbi {
    /// The list of ABIs.
    pub abis: ThinVec<LitStr>,
}
