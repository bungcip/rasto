use crate::ast::*;

pub fn file() -> FileBuilder {
    FileBuilder::new()
}

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

pub fn fn_def(name: impl Into<String>) -> FnBuilder {
    FnBuilder::new(name)
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

pub fn const_item(name: impl Into<String>, ty: impl Into<String>, expr: impl Into<Expr>) -> ItemConstBuilder {
    ItemConstBuilder::new(name, ty, expr)
}

pub struct ItemConstBuilder {
    ident: String,
    ty: String,
    expr: Box<Expr>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemConstBuilder {
    pub fn new(name: impl Into<String>, ty: impl Into<String>, expr: impl Into<Expr>) -> Self {
        Self {
            ident: name.into(),
            ty: ty.into(),
            expr: Box::new(expr.into()),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    pub fn build(self) -> ItemConst {
        ItemConst {
            ident: self.ident,
            ty: self.ty,
            expr: self.expr,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

pub fn extern_crate_item(name: impl Into<String>) -> ItemExternCrateBuilder {
    ItemExternCrateBuilder::new(name)
}

pub struct ItemExternCrateBuilder {
    ident: String,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemExternCrateBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    pub fn build(self) -> ItemExternCrate {
        ItemExternCrate {
            ident: self.ident,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

pub fn foreign_mod_item(abi: impl Into<String>) -> ItemForeignModBuilder {
    ItemForeignModBuilder::new(abi)
}

pub struct ItemForeignModBuilder {
    abi: String,
    items: Vec<Item>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemForeignModBuilder {
    pub fn new(abi: impl Into<String>) -> Self {
        Self {
            abi: abi.into(),
            items: vec![],
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    pub fn item(mut self, item: impl Into<Item>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    pub fn build(self) -> ItemForeignMod {
        ItemForeignMod {
            abi: self.abi,
            items: self.items,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

pub fn macro_item(expr: impl Into<Expr>) -> ItemMacroBuilder {
    ItemMacroBuilder::new(expr)
}

pub struct ItemMacroBuilder {
    expr: Expr,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemMacroBuilder {
    pub fn new(expr: impl Into<Expr>) -> Self {
        Self {
            expr: expr.into(),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    pub fn build(self) -> ItemMacro {
        ItemMacro {
            expr: self.expr,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

pub fn mod_item(name: impl Into<String>) -> ItemModBuilder {
    ItemModBuilder::new(name)
}

pub struct ItemModBuilder {
    ident: String,
    content: Option<Vec<Item>>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemModBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            content: None,
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    pub fn content(mut self, content: Vec<Item>) -> Self {
        self.content = Some(content);
        self
    }

    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    pub fn build(self) -> ItemMod {
        ItemMod {
            ident: self.ident,
            content: self.content,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

pub fn static_item(name: impl Into<String>, ty: impl Into<String>, expr: impl Into<Expr>) -> ItemStaticBuilder {
    ItemStaticBuilder::new(name, ty, expr)
}

pub struct ItemStaticBuilder {
    ident: String,
    ty: String,
    expr: Box<Expr>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemStaticBuilder {
    pub fn new(name: impl Into<String>, ty: impl Into<String>, expr: impl Into<Expr>) -> Self {
        Self {
            ident: name.into(),
            ty: ty.into(),
            expr: Box::new(expr.into()),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    pub fn build(self) -> ItemStatic {
        ItemStatic {
            ident: self.ident,
            ty: self.ty,
            expr: self.expr,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

pub fn trait_alias_item(name: impl Into<String>, bounds: Vec<String>) -> ItemTraitAliasBuilder {
    ItemTraitAliasBuilder::new(name, bounds)
}

pub struct ItemTraitAliasBuilder {
    ident: String,
    bounds: Vec<String>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemTraitAliasBuilder {
    pub fn new(name: impl Into<String>, bounds: Vec<String>) -> Self {
        Self {
            ident: name.into(),
            bounds,
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    pub fn build(self) -> ItemTraitAlias {
        ItemTraitAlias {
            ident: self.ident,
            bounds: self.bounds,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

pub fn type_item(name: impl Into<String>, ty: impl Into<String>) -> ItemTypeBuilder {
    ItemTypeBuilder::new(name, ty)
}

pub struct ItemTypeBuilder {
    ident: String,
    ty: String,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemTypeBuilder {
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            ty: ty.into(),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    pub fn build(self) -> ItemType {
        ItemType {
            ident: self.ident,
            ty: self.ty,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

pub fn union_item(name: impl Into<String>) -> ItemUnionBuilder {
    ItemUnionBuilder::new(name)
}

pub struct ItemUnionBuilder {
    ident: String,
    fields: Vec<Field>,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemUnionBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            ident: name.into(),
            fields: vec![],
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    pub fn build(self) -> ItemUnion {
        ItemUnion {
            ident: self.ident,
            fields: self.fields,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}

pub fn use_item(path: impl Into<String>) -> ItemUseBuilder {
    ItemUseBuilder::new(path)
}

pub struct ItemUseBuilder {
    path: String,
    leading_comments: Vec<Comment>,
    trailing_comments: Vec<Comment>,
}

impl ItemUseBuilder {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            leading_comments: vec![],
            trailing_comments: vec![],
        }
    }

    pub fn leading_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.leading_comments.push(comment.into());
        self
    }

    pub fn trailing_comment(mut self, comment: impl Into<Comment>) -> Self {
        self.trailing_comments.push(comment.into());
        self
    }

    pub fn build(self) -> ItemUse {
        ItemUse {
            path: self.path,
            leading_comments: self.leading_comments,
            trailing_comments: self.trailing_comments,
        }
    }
}
