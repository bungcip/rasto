use rasto::ast::*;

#[test]
fn test_lit_from_string() {
    let lit: Lit = "hello".to_string().into();
    assert!(matches!(lit, Lit::Str(_)));
}

#[test]
fn test_lit_from_str() {
    let lit: Lit = "hello".into();
    assert!(matches!(lit, Lit::Str(_)));
}

#[test]
fn test_lit_from_u64() {
    let lit: Lit = 42u64.into();
    assert!(matches!(lit, Lit::Int(_)));
}

#[test]
fn test_lit_from_i32() {
    let lit: Lit = 42i32.into();
    assert!(matches!(lit, Lit::Int(_)));
}

#[test]
fn test_lit_from_bool() {
    let lit: Lit = true.into();
    assert!(matches!(lit, Lit::Bool(_)));
}

#[test]
fn test_lit_from_f64() {
    let lit: Lit = 1.23f64.into();
    assert!(matches!(lit, Lit::Float(_)));
}

#[test]
fn test_lit_from_char() {
    let lit: Lit = 'h'.into();
    assert!(matches!(lit, Lit::Char(_)));
}

#[test]
fn test_lit_from_u8() {
    let lit: Lit = b'h'.into();
    assert!(matches!(lit, Lit::Byte(_)));
}

#[test]
fn test_lit_from_u8_slice() {
    let lit: Lit = b"hello".as_slice().into();
    assert!(matches!(lit, Lit::ByteStr(_)));
}

#[test]
fn test_lit_from_u8_array() {
    let lit: Lit = (&[b'h', b'e', b'l', b'l', b'o'] as &[u8]).into();
    assert!(matches!(lit, Lit::ByteStr(_)));
}
