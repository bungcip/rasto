use rasto::ast::{builder::fn_def, Block, Expr, Lit, Stmt};
use std::fmt;

struct TestRope {
    buffer: String,
    indent_level: usize,
}

impl TestRope {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            indent_level: 0,
        }
    }
}

impl fmt::Write for TestRope {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (i, line) in s.split('\n').enumerate() {
            if i > 0 {
                self.buffer.push('\n');
            }

            if !line.is_empty() {
                for _ in 0..self.indent_level {
                    self.buffer.push_str("    ");
                }
                self.buffer.push_str(line);
            }
        }
        Ok(())
    }
}

#[test]
fn test_fn_builder() {
    let item_fn = fn_def("foo")
        .block(Block {
            leading_comments: vec![],
            stmts: vec![Stmt::Expr(Expr::Lit(Lit::Str(
                "Hello, world!".to_string(),
            )))],
            trailing_comments: vec![],
        })
        .build();

    let actual = item_fn.to_string();

    insta::assert_snapshot!(actual);
}
