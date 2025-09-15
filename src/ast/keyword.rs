//! Defines the AST nodes for keywords.

/// The `asm` keyword, used for inline assembly.
#[doc = "```rust,ignore\nasm!(\"...\");\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Asm;

/// The `lateout` keyword, used in `asm!` expressions.
#[doc = "```rust,ignore\nasm!(\"\", out(reg) _, lateout(reg) _);\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lateout;

/// The `inlateout` keyword, used in `asm!` expressions.
#[doc = "```rust,ignore\nasm!(\"\", inlateout(reg) _);\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Inlateout;

/// The `options` keyword, used in `asm!` expressions.
#[doc = "```rust,ignore\nasm!(\"\", options(nostack));\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Options;

/// The `pure` keyword, used in `asm!` options.
#[doc = "```rust,ignore\nasm!(\"\", options(pure));\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pure;

/// The `nomem` keyword, used in `asm!` options.
#[doc = "```rust,ignore\nasm!(\"\", options(nomem));\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Nomem;

/// The `readonly` keyword, used in `asm!` options.
#[doc = "```rust,ignore\nasm!(\"\", options(readonly));\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Readonly;

/// The `preserves_flags` keyword, used in `asm!` options.
#[doc = "```rust,ignore\nasm!(\"\", options(preserves_flags));\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreservesFlags;

/// The `noreturn` keyword, used in `asm!` options.
#[doc = "```rust,ignore\nasm!(\"\", options(noreturn));\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Noreturn;

/// The `nostack` keyword, used in `asm!` options.
#[doc = "```rust,ignore\nasm!(\"\", options(nostack));\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Nostack;

/// The `att_syntax` keyword, used in `asm!` options.
#[doc = "```rust,ignore\nasm!(\"\", options(att_syntax));\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttSyntax;

/// The `raw` keyword, used in `asm!` options.
#[doc = "```rust,ignore\nasm!(\"\", options(raw));\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Raw;

/// The `clobber_abi` keyword, used in `asm!` expressions.
#[doc = "```rust,ignore\nasm!(\"\", clobber_abi(\"C\"));\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClobberAbi;

/// The `in` keyword, used in `asm!` expressions.
#[doc = "```rust,ignore\nasm!(\"\", in(reg) _);\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct In;

/// The `out` keyword, used in `asm!` expressions.
#[doc = "```rust,ignore\nasm!(\"\", out(reg) _);\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Out;

/// The `inout` keyword, used in `asm!` expressions.
#[doc = "```rust,ignore\nasm!(\"\", inout(reg) _);\n```"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Inout;
