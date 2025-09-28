use rasto::ast::{AsmDirection, LitStr, RegSpec};
use rasto::builder::{asm_item, asm_operand, expr, fn_def, stmt};
use rasto::pretty;

#[test]
fn test_asm_macro() {
    let ast = fn_def("test")
        .unsafe_()
        .statement(
            stmt().item(
                asm_item(LitStr::new("jmp 3f"))
                    .template(LitStr::new("2: .ascii \\\"Hello World!\\\""))
                    .template(LitStr::new("3: lea {bytes}, [2b+rip]"))
                    .template(LitStr::new("mov {len}, 12"))
                    .operand(
                        asm_operand()
                            .reg(
                                AsmDirection::Out,
                                RegSpec::Class("reg".to_string()),
                                expr().path("bytes"),
                            )
                            .name("bytes"),
                    )
                    .operand(
                        asm_operand()
                            .reg(
                                AsmDirection::Out,
                                RegSpec::Class("reg".to_string()),
                                expr().path("len"),
                            )
                            .name("len"),
                    )
                    .build(),
            ),
        )
        .build();
    insta::assert_snapshot!(pretty(&ast));
}
