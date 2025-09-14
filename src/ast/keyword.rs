//! Defines the AST nodes for keywords.

/// The `asm` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Asm;

/// The `lateout` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lateout;

/// The `inlateout` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Inlateout;

/// The `options` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Options;

/// The `pure` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pure;

/// The `nomem` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Nomem;

/// The `readonly` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Readonly;

/// The `preserves_flags` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreservesFlags;

/// The `noreturn` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Noreturn;

/// The `nostack` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Nostack;

/// The `att_syntax` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttSyntax;

/// The `raw` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Raw;

/// The `clobber_abi` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClobberAbi;

/// The `in` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct In;

/// The `out` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Out;

/// The `inout` keyword.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Inout;
