use crate::ast::*;

pub struct FileBuilder {
    items: Vec<Item>,
}

impl FileBuilder {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn item(mut self, item: impl Into<Item>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn build(self) -> File {
        File { items: self.items }
    }
}

pub struct FnBuilder {
    ident: String,
    block: Option<Block>,
}

impl FnBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            block: None,
        }
    }

    pub fn block(mut self, block: Block) -> Self {
        self.block = Some(block);
        self
    }

    pub fn build(self) -> ItemFn {
        let block = self.block.expect("block is required");

        ItemFn {
            leading_comments: vec![],
            sig: Signature { ident: self.ident },
            block,
            trailing_comments: vec![],
        }
    }
}

pub fn expr() -> ExprBuilder {
    ExprBuilder
}

#[derive(Clone, Copy)]
pub struct ExprBuilder;

impl ExprBuilder {
    pub fn array(self, elems: impl IntoIterator<Item = Expr>) -> Expr {
        Expr::Array(ExprArray {
            elems: elems.into_iter().collect(),
        })
    }

    pub fn assign(self, left: Expr, right: Expr) -> Expr {
        Expr::Assign(ExprAssign {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn async_block(self, block: Block) -> Expr {
        Expr::Async(ExprAsync { block })
    }

    pub fn await_expr(self, expr: Expr) -> Expr {
        Expr::Await(ExprAwait {
            expr: Box::new(expr),
        })
    }

    pub fn binary(self, left: Expr, op: BinOp, right: Expr) -> Expr {
        Expr::Binary(ExprBinary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    pub fn block(self, block: Block) -> Expr {
        Expr::Block(ExprBlock { block })
    }

    pub fn break_expr(self) -> Expr {
        Expr::Break(ExprBreak)
    }

    pub fn call(self, func: Expr, args: impl IntoIterator<Item = Expr>) -> Expr {
        Expr::Call(ExprCall {
            func: Box::new(func),
            args: args.into_iter().collect(),
        })
    }

    pub fn cast(self, expr: Expr, ty: impl Into<String>) -> Expr {
        Expr::Cast(ExprCast {
            expr: Box::new(expr),
            ty: ty.into(),
        })
    }

    pub fn closure(
        self,
        inputs: impl IntoIterator<Item = impl Into<String>>,
        body: Expr,
    ) -> Expr {
        Expr::Closure(ExprClosure {
            inputs: inputs.into_iter().map(Into::into).collect(),
            body: Box::new(body),
        })
    }

    pub fn const_block(self, block: Block) -> Expr {
        Expr::Const(ExprConst { block })
    }

    pub fn continue_expr(self) -> Expr {
        Expr::Continue(ExprContinue)
    }

    pub fn field(self, expr: Expr, member: impl Into<String>) -> Expr {
        Expr::Field(ExprField {
            expr: Box::new(expr),
            member: member.into(),
        })
    }

    pub fn for_loop(self, pat: impl Into<String>, expr: Expr, body: Block) -> Expr {
        Expr::For(ExprFor {
            pat: pat.into(),
            expr: Box::new(expr),
            body,
        })
    }

    pub fn if_expr(self, cond: Expr, then_branch: Block, else_branch: Option<Expr>) -> Expr {
        Expr::If(ExprIf {
            cond: Box::new(cond),
            then_branch,
            else_branch: else_branch.map(Box::new),
        })
    }

    pub fn index(self, expr: Expr, index: Expr) -> Expr {
        Expr::Index(ExprIndex {
            expr: Box::new(expr),
            index: Box::new(index),
        })
    }

    pub fn lit(self, lit: impl Into<Lit>) -> Expr {
        Expr::Lit(lit.into())
    }

    pub fn loop_expr(self, body: Block) -> Expr {
        Expr::Loop(ExprLoop { body })
    }

    pub fn macro_call(self, ident: impl Into<String>, tokens: TokenStream) -> Expr {
        Expr::MacroCall(ExprMacroCall {
            ident: ident.into(),
            tokens,
        })
    }

    pub fn match_expr(self, expr: Expr, arms: impl IntoIterator<Item = Arm>) -> Expr {
        Expr::Match(ExprMatch {
            expr: Box::new(expr),
            arms: arms.into_iter().collect(),
        })
    }

    pub fn method_call(
        self,
        receiver: Expr,
        method: impl Into<String>,
        args: impl IntoIterator<Item = Expr>,
    ) -> Expr {
        Expr::MethodCall(ExprMethodCall {
            receiver: Box::new(receiver),
            method: method.into(),
            args: args.into_iter().collect(),
        })
    }

    pub fn paren(self, expr: Expr) -> Expr {
        Expr::Paren(ExprParen {
            expr: Box::new(expr),
        })
    }

    pub fn range(self, start: Option<Expr>, limits: RangeLimits, end: Option<Expr>) -> Expr {
        Expr::Range(ExprRange {
            start: start.map(Box::new),
            limits,
            end: end.map(Box::new),
        })
    }

    pub fn reference(self, is_mut: bool, expr: Expr) -> Expr {
        Expr::Reference(ExprRef {
            is_mut,
            expr: Box::new(expr),
        })
    }

    pub fn return_expr(self, expr: Option<Expr>) -> Expr {
        Expr::Return(ExprReturn {
            expr: expr.map(Box::new),
        })
    }

    pub fn struct_expr(
        self,
        path: impl Into<String>,
        fields: impl IntoIterator<Item = FieldValue>,
    ) -> Expr {
        Expr::Struct(ExprStruct {
            path: path.into(),
            fields: fields.into_iter().collect(),
        })
    }

    pub fn tuple(self, elems: impl IntoIterator<Item = Expr>) -> Expr {
        Expr::Tuple(ExprTuple {
            elems: elems.into_iter().collect(),
        })
    }

    pub fn while_loop(self, cond: Expr, body: Block) -> Expr {
        Expr::While(ExprWhile {
            cond: Box::new(cond),
            body,
        })
    }
}
