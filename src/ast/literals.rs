//! Defines the AST nodes for literals.
//!
//! Literals are values that are written directly in the source code, such as strings,
//! numbers, and booleans.

use std::str::FromStr;

/// A literal expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lit {
    /// A string literal, e.g., `"hello"`.
    Str(LitStr),
    /// A byte string literal, e.g., `b"hello"`.
    ByteStr(LitByteStr),
    /// A C-string literal, e.g., `c"hello"`.
    CStr(LitCStr),
    /// A byte literal, e.g., `b'h'`.
    Byte(LitByte),
    /// A character literal, e.g., `'h'`.
    Char(LitChar),
    /// An integer literal, e.g., `42`.
    Int(LitInt),
    /// A float literal, e.g., `1.23`.
    Float(LitFloat),
    /// A boolean literal, e.g., `true` or `false`.
    Bool(LitBool),
}

/// A string literal, e.g., `"hello"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LitStr {
    /// The value of the string literal.
    pub value: String,
}

impl LitStr {
    /// Creates a new `LitStr`.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value.
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl FromStr for LitStr {
    type Err = ();

    /// Creates a new `LitStr` from a string slice.
    ///
    /// The string slice must be enclosed in double quotes.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') && s.ends_with('"') {
            Ok(LitStr {
                value: s[1..s.len() - 1].to_string(),
            })
        } else {
            Err(())
        }
    }
}

impl From<&str> for LitStr {
    /// Creates a new `LitStr` from a string slice.
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

/// A byte string literal, e.g., `b"hello"`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitByteStr {
    /// The value of the byte string literal.
    pub value: Vec<u8>,
}

impl LitByteStr {
    /// Creates a new `LitByteStr`.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte string value.
    pub fn new(value: &[u8]) -> Self {
        Self {
            value: value.to_vec(),
        }
    }
}

/// A C-string literal, e.g., `c"hello"`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitCStr {
    /// The value of the C-string literal.
    pub value: Vec<u8>,
}

impl LitCStr {
    /// Creates a new `LitCStr`.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value.
    pub fn new(value: &str) -> Self {
        Self {
            value: value.as_bytes().to_vec(),
        }
    }
}

/// A byte literal, e.g., `b'h'`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitByte {
    /// The value of the byte literal.
    pub value: u8,
}

impl LitByte {
    /// Creates a new `LitByte`.
    ///
    /// # Arguments
    ///
    /// * `value` - The byte value.
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

/// A character literal, e.g., `'h'`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitChar {
    /// The value of the character literal.
    pub value: char,
}

impl LitChar {
    /// Creates a new `LitChar`.
    ///
    /// # Arguments
    ///
    /// * `value` - The character value.
    pub fn new(value: char) -> Self {
        Self { value }
    }
}

/// The suffix of an integer literal, e.g., `u32`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntSuffix {
    /// `u8` suffix.
    U8,
    /// `i8` suffix.
    I8,
    /// `u16` suffix.
    U16,
    /// `i16` suffix.
    I16,
    /// `u32` suffix.
    U32,
    /// `i32` suffix.
    I32,
    /// `u64` suffix.
    U64,
    /// `i64` suffix.
    I64,
    /// `u128` suffix.
    U128,
    /// `i128` suffix.
    I128,
    /// `usize` suffix.
    Usize,
    /// `isize` suffix.
    Isize,
}

/// An integer literal, e.g., `42`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitInt {
    /// The value of the integer literal.
    pub value: u128,
    /// The suffix of the integer literal, e.g., `u32`.
    pub suffix: Option<IntSuffix>,
}

impl LitInt {
    /// Creates a new `LitInt`.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value.
    pub fn new(value: u128) -> Self {
        Self {
            value,
            suffix: None,
        }
    }

    /// Adds a suffix to the `LitInt`.
    ///
    /// # Arguments
    ///
    /// * `suffix` - The integer suffix.
    pub fn with_suffix(value: u128, suffix: IntSuffix) -> Self {
        Self {
            value,
            suffix: Some(suffix),
        }
    }
}

/// The suffix of a float literal, e.g., `f64`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FloatSuffix {
    /// `f32` suffix.
    F32,
    /// `f64` suffix.
    F64,
}

/// A float literal, e.g., `1.23`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitFloat {
    /// The value of the float literal.
    pub value: String,
    /// The suffix of the float literal, e.g., `f64`.
    pub suffix: Option<FloatSuffix>,
}

impl LitFloat {
    /// Creates a new `LitFloat`.
    ///
    /// # Arguments
    ///
    /// * `value` - The float value as a string.
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            suffix: None,
        }
    }

    /// Adds a suffix to the `LitFloat`.
    ///
    /// # Arguments
    ///
    /// * `suffix` - The float suffix.
    pub fn with_suffix(value: &str, suffix: FloatSuffix) -> Self {
        Self {
            value: value.to_string(),
            suffix: Some(suffix),
        }
    }
}

/// A boolean literal, e.g., `true` or `false`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LitBool {
    /// The value of the boolean literal.
    pub value: bool,
}

impl LitBool {
    /// Creates a new `LitBool`.
    ///
    /// # Arguments
    ///
    /// * `value` - The boolean value.
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl From<String> for Lit {
    /// Converts a `String` into a `Lit::Str` variant.
    fn from(s: String) -> Self {
        Lit::Str(LitStr { value: s })
    }
}

impl From<&str> for Lit {
    /// Converts a `&str` into a `Lit::Str` variant.
    fn from(s: &str) -> Self {
        Lit::Str(LitStr {
            value: s.to_string(),
        })
    }
}

impl From<u64> for Lit {
    /// Converts a `u64` into a `Lit::Int` variant.
    fn from(i: u64) -> Self {
        Lit::Int(LitInt {
            value: i as u128,
            suffix: None,
        })
    }
}

impl From<i32> for Lit {
    /// Converts an `i32` into a `Lit::Int` variant.
    fn from(i: i32) -> Self {
        Lit::Int(LitInt {
            value: i as u128,
            suffix: None,
        })
    }
}

impl From<bool> for Lit {
    /// Converts a `bool` into a `Lit::Bool` variant.
    fn from(b: bool) -> Self {
        Lit::Bool(LitBool { value: b })
    }
}

impl From<f64> for Lit {
    /// Converts a `f64` into a `Lit::Float` variant.
    fn from(f: f64) -> Self {
        Lit::Float(LitFloat::new(&f.to_string()))
    }
}

impl From<char> for Lit {
    /// Converts a `char` into a `Lit::Char` variant.
    fn from(c: char) -> Self {
        Lit::Char(LitChar::new(c))
    }
}

impl From<u8> for Lit {
    /// Converts a `u8` into a `Lit::Byte` variant.
    fn from(b: u8) -> Self {
        Lit::Byte(LitByte::new(b))
    }
}

impl From<&[u8]> for Lit {
    /// Converts a `&[u8]` into a `Lit::ByteStr` variant.
    fn from(s: &[u8]) -> Self {
        Lit::ByteStr(LitByteStr::new(s))
    }
}

impl<const N: usize> From<&[u8; N]> for Lit {
    /// Converts a `&[u8; N]` into a `Lit::ByteStr` variant.
    fn from(array: &[u8; N]) -> Self {
        Lit::ByteStr(LitByteStr::new(array.as_slice()))
    }
}

impl<'a> From<&'a std::ffi::CStr> for Lit {
    /// Converts a `&CStr` into a `Lit::CStr` variant.
    fn from(s: &'a std::ffi::CStr) -> Self {
        Lit::CStr(LitCStr::new(s.to_str().unwrap()))
    }
}
