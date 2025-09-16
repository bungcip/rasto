use rasto::ast::*;

#[test]
fn test_from_impls() {
    let lit: Lit = "hello".to_string().into();
    assert!(matches!(lit, Lit::Str(_)));

    let lit: Lit = "hello".into();
    assert!(matches!(lit, Lit::Str(_)));

    let lit: Lit = 42u64.into();
    assert!(matches!(lit, Lit::Int(_)));

    let lit: Lit = 42i32.into();
    assert!(matches!(lit, Lit::Int(_)));

    let lit: Lit = true.into();
    assert!(matches!(lit, Lit::Bool(_)));

    let lit: Lit = 1.23f64.into();
    assert!(matches!(lit, Lit::Float(_)));

    let lit: Lit = 'h'.into();
    assert!(matches!(lit, Lit::Char(_)));

    let lit: Lit = b'h'.into();
    assert!(matches!(lit, Lit::Byte(_)));

    let lit: Lit = b"hello".as_slice().into();
    assert!(matches!(lit, Lit::ByteStr(_)));

    let lit: Lit = (&[b'h', b'e', b'l', b'l', b'o'] as &[u8]).into();
    assert!(matches!(lit, Lit::ByteStr(_)));
}
