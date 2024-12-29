use rustpython_ast::{
    text_size::TextRange, Alias, Arg, Arguments, BoolOp, CmpOp, Comprehension, ExceptHandler,
    ExceptHandlerExceptHandler, Expr, ExprAttribute, ExprAwait, ExprBinOp, ExprBoolOp, ExprCall,
    ExprCompare, ExprConstant, ExprDict, ExprDictComp, ExprFormattedValue, ExprGeneratorExp,
    ExprIfExp, ExprJoinedStr, ExprLambda, ExprList, ExprListComp, ExprName, ExprNamedExpr, ExprSet,
    ExprSetComp, ExprSlice, ExprStarred, ExprSubscript, ExprTuple, ExprUnaryOp, ExprYield,
    ExprYieldFrom, Keyword, MatchCase, Operator, Pattern, PatternMatchAs, PatternMatchClass,
    PatternMatchMapping, PatternMatchOr, PatternMatchSequence, PatternMatchSingleton,
    PatternMatchStar, PatternMatchValue, Stmt, StmtAnnAssign, StmtAssert, StmtAssign, StmtAsyncFor,
    StmtAsyncFunctionDef, StmtAsyncWith, StmtAugAssign, StmtBreak, StmtClassDef, StmtContinue,
    StmtDelete, StmtExpr, StmtFor, StmtFunctionDef, StmtGlobal, StmtIf, StmtImport, StmtImportFrom,
    StmtMatch, StmtNonlocal, StmtPass, StmtRaise, StmtReturn, StmtTry, StmtTryStar, StmtTypeAlias,
    StmtWhile, StmtWith, TypeParam, TypeParamParamSpec, TypeParamTypeVar, TypeParamTypeVarTuple,
    UnaryOp, WithItem,
};
use rustpython_ast::{Constant, Int};
use std::fmt;
use std::fmt::Formatter;

fn get_precedence(node: &Expr<TextRange>) -> usize {
    match node {
        Expr::NamedExpr(_) => 1,
        Expr::Tuple(_) => 2,
        Expr::Yield(_) => 3,
        Expr::YieldFrom(_) => 3,
        Expr::IfExp(_) => 4,
        Expr::Lambda(_) => 4,
        Expr::BoolOp(data) => match data.op {
            BoolOp::Or => 5,
            BoolOp::And => 6,
        },
        Expr::UnaryOp(data) => match data.op {
            UnaryOp::Not => 7,
            UnaryOp::UAdd => 15,
            UnaryOp::USub => 15,
            UnaryOp::Invert => 15,
        },
        Expr::Compare(_) => 8,
        Expr::BinOp(data) => match data.op {
            Operator::BitOr => 9,
            Operator::BitXor => 10,
            Operator::BitAnd => 11,
            Operator::LShift => 12,
            Operator::RShift => 12,
            Operator::Add => 13,
            Operator::Sub => 13,
            Operator::Div => 14,
            Operator::FloorDiv => 14,
            Operator::Mult => 14,
            Operator::MatMult => 14,
            Operator::Mod => 14,
            Operator::Pow => 16,
        },
        Expr::Await(_) => 17,
        _ => 4,
    }
}

pub struct Unparser<'a> {
    fmt: Formatter<'a>,
    _indent: usize,
    _in_try_star: bool,
    _precedence_level: usize,
}

impl<'a> Unparser<'a> {
    pub fn new(fmt: Formatter<'a>) -> Self {
        Unparser {
            _in_try_star: false,
            _indent: 0,
            _precedence_level: 0,
            fmt,
        }
    }

    fn fill(&mut self, str_: &str) -> fmt::Result {
        self.write_str(&("\n".to_owned() + &" ".repeat(self._indent * 4) + str_))
    }

    fn write_str(&mut self, str_: &str) -> fmt::Result {
        self.fmt.write_str(str_)
    }

    fn block<F>(&mut self, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Self) -> fmt::Result,
    {
        self._indent += 1;
        f(self)?;
        self._indent -= 1;
        Ok(())
    }

    fn delimit_precedence<F>(&mut self, node: &Expr<TextRange>, f: F) -> fmt::Result
    where
        F: FnOnce(&mut Self) -> fmt::Result,
    {
        self._precedence_level = get_precedence(node);
        let should_delimit = get_precedence(node) > self._precedence_level;
        if should_delimit {
            self.write_str("(")?;
        }
        f(self)?;
        if should_delimit {
            self.write_str(")")?;
        }
        Ok(())
    }

    pub fn unparse_stmt(&mut self, node: &Stmt<TextRange>) -> fmt::Result {
        match node {
            Stmt::FunctionDef(data) => self.unparse_stmt_function_def(data),
            Stmt::AsyncFunctionDef(data) => self.unparse_stmt_async_function_def(data),
            Stmt::ClassDef(data) => self.unparse_stmt_class_def(data),
            Stmt::Return(data) => self.unparse_stmt_return(data),
            Stmt::Delete(data) => self.unparse_stmt_delete(data),
            Stmt::Assign(data) => self.unparse_stmt_assign(data),
            Stmt::TypeAlias(data) => self.unparse_stmt_type_alias(data),
            Stmt::AugAssign(data) => self.unparse_stmt_aug_assign(data),
            Stmt::AnnAssign(data) => self.unparse_stmt_ann_assign(data),
            Stmt::For(data) => self.unparse_stmt_for(data),
            Stmt::AsyncFor(data) => self.unparse_stmt_async_for(data),
            Stmt::While(data) => self.unparse_stmt_while(data),
            Stmt::If(data) => self.unparse_stmt_if(data),
            Stmt::With(data) => self.unparse_stmt_with(data),
            Stmt::AsyncWith(data) => self.unparse_stmt_async_with(data),
            Stmt::Match(data) => self.unparse_stmt_match(data),
            Stmt::Raise(data) => self.unparse_stmt_raise(data),
            Stmt::Try(data) => self.unparse_stmt_try(data),
            Stmt::TryStar(data) => self.unparse_stmt_try_star(data),
            Stmt::Assert(data) => self.unparse_stmt_assert(data),
            Stmt::Import(data) => self.unparse_stmt_import(data),
            Stmt::ImportFrom(data) => self.unparse_stmt_import_from(data),
            Stmt::Global(data) => self.unparse_stmt_global(data),
            Stmt::Nonlocal(data) => self.unparse_stmt_nonlocal(data),
            Stmt::Expr(data) => self.unparse_stmt_expr(data),
            Stmt::Pass(data) => self.unparse_stmt_pass(data),
            Stmt::Break(data) => self.unparse_stmt_break(data),
            Stmt::Continue(data) => self.unparse_stmt_continue(data),
        }
    }

    fn unparse_stmt_pass(&mut self, _node: &StmtPass<TextRange>) -> fmt::Result {
        self.fill("pass")
    }

    fn unparse_stmt_break(&mut self, _node: &StmtBreak<TextRange>) -> fmt::Result {
        self.fill("break")
    }

    fn unparse_stmt_continue(&mut self, _node: &StmtContinue<TextRange>) -> fmt::Result {
        self.fill("continue")
    }

    fn unparse_stmt_function_def(&mut self, node: &StmtFunctionDef<TextRange>) -> fmt::Result {
        for decorator in &node.decorator_list {
            self.fill("@")?;
            self.unparse_expr(&decorator)?;
        }
        self.fill("def ")?;
        self.write_str(&node.name)?;
        if node.type_params.len() > 0 {
            self.write_str("[")?;
            let mut type_params_iter = node.type_params.iter().peekable();
            while let Some(type_param) = type_params_iter.next() {
                self.unparse_type_param(type_param)?;
                if type_params_iter.peek().is_some() {
                    self.write_str(", ")?;
                }
            }
            self.write_str("]")?;
        }
        self.write_str("(")?;

        self.unparse_arguments(&node.args)?;

        self.write_str(")")?;
        if let Some(returns) = &node.returns {
            self.write_str(" -> ")?;
            self.unparse_expr(&returns)?;
        }
        self.write_str(":")?;
        self.block(|block_self| {
            for value in &node.body {
                block_self.unparse_stmt(&value)?;
            }
            Ok(())
        })?;
        Ok(())
    }

    fn unparse_stmt_async_function_def(
        &mut self,
        node: &StmtAsyncFunctionDef<TextRange>,
    ) -> fmt::Result {
        for decorator in &node.decorator_list {
            self.fill("@")?;
            self.unparse_expr(&decorator)?;
        }
        self.fill("async def ")?;
        self.write_str(&node.name)?;
        if node.type_params.len() > 0 {
            self.write_str("[")?;
            let mut type_params_iter = node.type_params.iter().peekable();
            while let Some(type_param) = type_params_iter.next() {
                self.unparse_type_param(type_param)?;
                if type_params_iter.peek().is_some() {
                    self.write_str(", ")?;
                }
            }
            self.write_str("]")?;
        }
        self.write_str("(")?;

        self.unparse_arguments(&node.args)?;

        self.write_str(")")?;
        if let Some(returns) = &node.returns {
            self.write_str(" -> ")?;
            self.unparse_expr(&returns)?;
        }
        self.write_str(":")?;
        self.block(|block_self| {
            for value in &node.body {
                block_self.unparse_stmt(&value)?;
            }
            Ok(())
        })?;
        Ok(())
    }

    fn unparse_stmt_class_def(&mut self, node: &StmtClassDef<TextRange>) -> fmt::Result {
        for decorator in &node.decorator_list {
            self.fill("@")?;
            self.unparse_expr(decorator)?;
        }

        self.fill("class")?;
        self.write_str(&node.name)?;

        if node.type_params.len() > 0 {
            self.write_str("[")?;
            let mut type_params_iter = node.type_params.iter().peekable();
            while let Some(type_param) = type_params_iter.next() {
                self.unparse_type_param(type_param)?;
                if type_params_iter.peek().is_some() {
                    self.write_str(", ")?;
                }
            }
            self.write_str("]")?;
        }

        self.write_str("(")?;

        let mut bases_iter = node.bases.iter().peekable();
        let mut keywords_iter = node.keywords.iter().peekable();

        while let Some(base) = bases_iter.next() {
            self.unparse_expr(base)?;
            if bases_iter.peek().is_some() || keywords_iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        while let Some(keyword) = keywords_iter.next() {
            self.unparse_keyword(keyword)?;
            if keywords_iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        self.write_str("):")?;

        self.block(|block_self| {
            for value in &node.body {
                block_self.unparse_stmt(&value)?;
            }
            Ok(())
        })?;
        Ok(())
    }

    fn unparse_stmt_return(&mut self, node: &StmtReturn<TextRange>) -> fmt::Result {
        self.fill("return ")?;
        if let Some(value) = &node.value {
            self.unparse_expr(&value)?;
        }
        Ok(())
    }
    fn unparse_stmt_delete(&mut self, node: &StmtDelete<TextRange>) -> fmt::Result {
        self.fill("del ")?;
        let mut targets_iter = node.targets.iter().peekable();

        while let Some(target) = targets_iter.next() {
            self.unparse_expr(target)?;
            if targets_iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        Ok(())
    }

    fn unparse_stmt_assign(&mut self, node: &StmtAssign<TextRange>) -> fmt::Result {
        let mut targets_iter = node.targets.iter().peekable();
        self.fill("")?;
        while let Some(target) = targets_iter.next() {
            self.unparse_expr(target)?;
            if targets_iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        self.write_str(" = ")?;
        self.unparse_expr(&node.value)?;
        if node.type_comment.is_some() {
            self.write_str(&node.type_comment.as_ref().unwrap())?;
        }
        Ok(())
    }

    fn unparse_stmt_type_alias(&mut self, node: &StmtTypeAlias<TextRange>) -> fmt::Result {
        self.fill("")?;
        self.unparse_expr(&node.name)?;
        if node.type_params.len() > 0 {
            self.write_str("[")?;
            let mut type_params_iter = node.type_params.iter().peekable();
            while let Some(type_param) = type_params_iter.next() {
                self.unparse_type_param(type_param)?;
                if type_params_iter.peek().is_some() {
                    self.write_str(", ")?;
                }
            }
            self.write_str("]")?;
        }
        self.write_str(": ")?;
        self.unparse_expr(&node.value)?;
        Ok(())
    }

    fn unparse_stmt_aug_assign(&mut self, node: &StmtAugAssign<TextRange>) -> fmt::Result {
        self.fill("")?;
        self.unparse_expr(&node.target)?;
        self.write_str(" ")?;
        self.unparse_operator(&node.op)?;
        self.write_str("= ")?;
        self.unparse_expr(&node.value)?;
        Ok(())
    }

    fn unparse_stmt_ann_assign(&mut self, node: &StmtAnnAssign<TextRange>) -> fmt::Result {
        self.fill("")?;
        self.unparse_expr(&node.target)?;
        self.write_str(": ")?;
        self.unparse_expr(&node.annotation)?;
        self.write_str(" = ")?;
        if let Some(value) = &node.value {
            self.unparse_expr(value)?;
        }
        Ok(())
    }

    fn unparse_stmt_for(&mut self, node: &StmtFor<TextRange>) -> fmt::Result {
        self.fill("for ")?;
        self.unparse_expr(&node.target)?;
        self.write_str(" in ")?;
        self.unparse_expr(&node.iter)?;
        self.write_str(":")?;
        self.block(|block_self| {
            for value in &node.body {
                block_self.unparse_stmt(value)?;
            }
            Ok(())
        })?;
        if node.orelse.len() > 0 {
            self.fill("else:")?;
            self.block(|block_self| {
                for stmt in &node.orelse {
                    block_self.unparse_stmt(stmt)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    }
    fn unparse_stmt_async_for(&mut self, node: &StmtAsyncFor<TextRange>) -> fmt::Result {
        self.fill("async for ")?;
        self.unparse_expr(&node.target)?;
        self.write_str(" in ")?;
        self.unparse_expr(&node.iter)?;
        self.write_str(":")?;
        self.block(|block_self| {
            for value in &node.body {
                block_self.unparse_stmt(value)?;
            }
            Ok(())
        })?;
        if node.orelse.len() > 0 {
            self.fill("else:")?;
            self.block(|block_self| {
                for stmt in &node.orelse {
                    block_self.unparse_stmt(stmt)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    }
    fn unparse_stmt_while(&mut self, node: &StmtWhile<TextRange>) -> fmt::Result {
        self.fill("while ")?;
        self.unparse_expr(&node.test)?;
        self.write_str(":")?;
        self.block(|block_self| {
            for stmt in &node.body {
                block_self.unparse_stmt(stmt)?;
            }
            Ok(())
        })?;

        if node.orelse.len() > 0 {
            self.fill("else:")?;
            self.block(|block_self| {
                for stmt in &node.orelse {
                    block_self.unparse_stmt(stmt)?;
                }
                Ok(())
            })?;
        }
        Ok(())
    }

    fn unparse_stmt_if(&mut self, node: &StmtIf<TextRange>) -> fmt::Result {
        self.fill("if ")?;
        self.unparse_expr(&node.test)?;
        self.write_str(":")?;
        self.block(|block_self| {
            for stmt in &node.body {
                block_self.unparse_stmt(stmt)?;
            }
            Ok(())
        })?;
        match node.orelse.as_slice() {
            [Stmt::If(inner_if)] => {
                self.fill("elif ")?;
                self.unparse_expr(&inner_if.test)?;
                self.write_str(":")?;
                self.block(|block_self| {
                    for stmt in &inner_if.body {
                        block_self.unparse_stmt(stmt)?;
                    }
                    Ok(())
                })?;
            }
            [] => {}
            _ => {
                self.fill("else:")?;
                self.block(|block_self| {
                    for stmt in &node.orelse {
                        block_self.unparse_stmt(stmt)?;
                    }
                    Ok(())
                })?;
            }
        }
        Ok(())
    }

    fn unparse_stmt_with(&mut self, node: &StmtWith<TextRange>) -> fmt::Result {
        self.fill("with ")?;
        let mut items_iter = node.items.iter().peekable();
        while let Some(item) = items_iter.next() {
            self.unparse_withitem(item)?;
            if items_iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        self.write_str(":")?;
        self.block(|block_self| {
            for stmt in &node.body {
                block_self.unparse_stmt(stmt)?;
            }
            Ok(())
        })?;
        Ok(())
    }
    fn unparse_stmt_async_with(&mut self, node: &StmtAsyncWith<TextRange>) -> fmt::Result {
        self.fill("async with ")?;
        let mut items_iter = node.items.iter().peekable();
        while let Some(item) = items_iter.next() {
            self.unparse_withitem(item)?;
            if items_iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        self.write_str(":")?;
        self.block(|block_self| {
            for stmt in &node.body {
                block_self.unparse_stmt(stmt)?;
            }
            Ok(())
        })?;
        Ok(())
    }

    fn unparse_stmt_match(&mut self, node: &StmtMatch<TextRange>) -> fmt::Result {
        self.fill("match ")?;
        self.unparse_expr(&node.subject)?;
        self.write_str(":")?;
        self.block(|block_self| {
            for case in &node.cases {
                block_self.unparse_match_case(case)?;
            }
            Ok(())
        })?;
        Ok(())
    }

    fn unparse_stmt_raise(&mut self, node: &StmtRaise<TextRange>) -> fmt::Result {
        self.fill("raise ")?;
        if let Some(exc) = &node.exc {
            self.unparse_expr(exc)?;
        }
        if let Some(cause) = &node.cause {
            self.write_str(" from ")?;
            self.unparse_expr(cause)?;
        }
        Ok(())
    }

    fn unparse_stmt_try(&mut self, node: &StmtTry<TextRange>) -> fmt::Result {
        let prev_try_star = self._in_try_star;
        self._in_try_star = false;
        self.fill("try:")?;
        self.block(|block_self| {
            for stmt in &node.body {
                block_self.unparse_stmt(stmt)?;
            }
            Ok(())
        })?;

        for handler in &node.handlers {
            self.unparse_excepthandler(handler)?;
        }

        if node.orelse.len() > 0 {
            self.fill("else:")?;
            self.block(|block_self| {
                for stmt in &node.orelse {
                    block_self.unparse_stmt(stmt)?;
                }
                Ok(())
            })?;
        }

        if node.finalbody.len() > 0 {
            self.fill("finally:")?;
            self.block(|block_self| {
                for stmt in &node.finalbody {
                    block_self.unparse_stmt(stmt)?;
                }
                Ok(())
            })?;
        }
        self._in_try_star = prev_try_star;
        Ok(())
    }
    fn unparse_stmt_try_star(&mut self, node: &StmtTryStar<TextRange>) -> fmt::Result {
        let prev_try_star = self._in_try_star;
        self._in_try_star = true;
        self.fill("try:")?;
        self.block(|block_self| {
            for stmt in &node.body {
                block_self.unparse_stmt(stmt)?;
            }
            Ok(())
        })?;

        for handler in &node.handlers {
            self.unparse_excepthandler(handler)?;
        }

        if node.orelse.len() > 0 {
            self.fill("else:")?;
            self.block(|block_self| {
                for stmt in &node.orelse {
                    block_self.unparse_stmt(stmt)?;
                }
                Ok(())
            })?;
        }

        if node.finalbody.len() > 0 {
            self.fill("finally:")?;
            self.block(|block_self| {
                for stmt in &node.finalbody {
                    block_self.unparse_stmt(stmt)?;
                }
                Ok(())
            })?;
        }
        self._in_try_star = prev_try_star;
        Ok(())
    }
    fn unparse_stmt_assert(&mut self, node: &StmtAssert<TextRange>) -> fmt::Result {
        self.fill("assert ")?;
        self.unparse_expr(&node.test)?;
        if let Some(msg) = &node.msg {
            self.write_str(", ")?;
            self.unparse_expr(msg)?;
        }
        Ok(())
    }

    fn unparse_stmt_import(&mut self, node: &StmtImport<TextRange>) -> fmt::Result {
        self.fill("import ")?;
        let mut iter = node.names.iter().peekable();
        while let Some(name) = iter.next() {
            self.unparse_alias(name)?;
            if iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        Ok(())
    }
    fn unparse_stmt_import_from(&mut self, node: &StmtImportFrom<TextRange>) -> fmt::Result {
        self.fill("from ")?;
        let level = node.level.unwrap_or(Int::new(0));
        self.write_str(&".".repeat(level.to_usize()))?;
        let module = match &node.module {
            Some(name) => name.to_string(),
            None => "".to_string(),
        };
        self.write_str(&(module + " import "))?;
        let mut iter = node.names.iter().peekable();
        while let Some(name) = iter.next() {
            self.unparse_alias(name)?;
            if iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        Ok(())
    }
    fn unparse_stmt_global(&mut self, node: &StmtGlobal<TextRange>) -> fmt::Result {
        self.fill("global ")?;
        let mut iter = node.names.iter().peekable();
        while let Some(name) = iter.next() {
            self.write_str(name)?;
            if iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        Ok(())
    }
    fn unparse_stmt_nonlocal(&mut self, node: &StmtNonlocal<TextRange>) -> fmt::Result {
        self.fill("nonlocal ")?;
        let mut iter = node.names.iter().peekable();
        while let Some(name) = iter.next() {
            self.write_str(name)?;
            if iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        Ok(())
    }
    fn unparse_stmt_expr(&mut self, node: &StmtExpr<TextRange>) -> fmt::Result {
        self.fill("")?;
        self.unparse_expr(&node.value)?;
        Ok(())
    }

    fn unparse_expr(&mut self, node: &Expr<TextRange>) -> fmt::Result {
        match node {
            Expr::BoolOp(data) => self.unparse_expr_bool_op(data),
            Expr::NamedExpr(data) => self.unparse_expr_named_expr(data),
            Expr::BinOp(data) => self.unparse_expr_bin_op(data),
            Expr::UnaryOp(data) => self.unparse_expr_unary_op(data),
            Expr::Lambda(data) => self.unparse_expr_lambda(data),
            Expr::IfExp(data) => self.unparse_expr_if_exp(data),
            Expr::Dict(data) => self.unparse_expr_dict(data),
            Expr::Set(data) => self.unparse_expr_set(data),
            Expr::ListComp(data) => self.unparse_expr_list_comp(data),
            Expr::SetComp(data) => self.unparse_expr_set_comp(data),
            Expr::DictComp(data) => self.unparse_expr_dict_comp(data),
            Expr::GeneratorExp(data) => self.unparse_expr_generator_exp(data),
            Expr::Await(data) => self.unparse_expr_await(data),
            Expr::Yield(data) => self.unparse_expr_yield(data),
            Expr::YieldFrom(data) => self.unparse_expr_yield_from(data),
            Expr::Compare(data) => self.unparse_expr_compare(data),
            Expr::Call(data) => self.unparse_expr_call(data),
            Expr::FormattedValue(data) => self.unparse_expr_formatted_value(data),
            Expr::JoinedStr(data) => self.unparse_expr_joined_str(data),
            Expr::Constant(data) => self.unparse_expr_constant(data),
            Expr::Attribute(data) => self.unparse_expr_attribute(data),
            Expr::Subscript(data) => self.unparse_expr_subscript(data),
            Expr::Starred(data) => self.unparse_expr_starred(data),
            Expr::Name(data) => self.unparse_expr_name(data),
            Expr::List(data) => self.unparse_expr_list(data),
            Expr::Tuple(data) => self.unparse_expr_tuple(data),
            Expr::Slice(data) => self.unparse_expr_slice(data),
        }
    }

    fn unparse_expr_bool_op(&mut self, node: &ExprBoolOp<TextRange>) -> fmt::Result {
        let prev_precedence_level = self._precedence_level;
        let operator = match node.op {
            BoolOp::And => " and ",
            BoolOp::Or => " or ",
        };

        let enum_member = Expr::BoolOp(node.to_owned());

        let mut values_iter = node.values.iter().peekable();
        self.delimit_precedence(&enum_member, |block_self| {
            while let Some(expr) = values_iter.next() {
                block_self._precedence_level += 1;
                block_self.unparse_expr(expr)?;
                if values_iter.peek().is_some() {
                    block_self.write_str(&operator)?;
                }
            }
            Ok(())
        })?;

        self._precedence_level = prev_precedence_level;
        Ok(())
    }

    fn unparse_expr_named_expr(&mut self, node: &ExprNamedExpr<TextRange>) -> fmt::Result {
        let enum_member = Expr::NamedExpr(node.to_owned());
        self.delimit_precedence(&enum_member, |block_self| {
            block_self.unparse_expr(&node.target)?;
            block_self.write_str(" := ")?;
            block_self.unparse_expr(&node.value)?;
            Ok(())
        })
    }

    fn unparse_expr_bin_op(&mut self, node: &ExprBinOp<TextRange>) -> fmt::Result {
        let enum_member = Expr::BinOp(node.to_owned());

        self.delimit_precedence(&enum_member, |block_self| {
            block_self.unparse_expr(&node.left)?;
            block_self.write_str(" ")?;
            block_self.unparse_operator(&node.op)?;
            block_self.write_str(" ")?;
            block_self.unparse_expr(&node.right)?;
            Ok(())
        })
    }

    fn unparse_expr_unary_op(&mut self, node: &ExprUnaryOp<TextRange>) -> fmt::Result {
        let enum_member = Expr::UnaryOp(node.to_owned());
        let operator = match node.op {
            UnaryOp::Invert => "~",
            UnaryOp::Not => "not ",
            UnaryOp::UAdd => "+",
            UnaryOp::USub => "-",
        };

        self.delimit_precedence(&enum_member, |block_self| {
            block_self.write_str(&operator)?;
            block_self.unparse_expr(&node.operand)
        })
    }
    fn unparse_expr_lambda(&mut self, node: &ExprLambda<TextRange>) -> fmt::Result {
        let enum_member = Expr::Lambda(node.to_owned());

        self.delimit_precedence(&enum_member, |block_self| {
            block_self.write_str("lambda ")?;
            block_self.unparse_arguments(&node.args)?;
            block_self.write_str(": ")?;
            block_self.unparse_expr(&node.body)?;
            Ok(())
        })
    }
    fn unparse_expr_if_exp(&mut self, node: &ExprIfExp<TextRange>) -> fmt::Result {
        let enum_member = Expr::IfExp(node.to_owned());
        self.delimit_precedence(&enum_member, |block_self| {
            block_self.unparse_expr(&node.body)?;
            block_self.write_str(" if ")?;
            block_self.unparse_expr(&node.test)?;
            block_self.write_str(" else ")?;
            block_self.unparse_expr(&node.orelse)?;
            Ok(())
        })
    }

    fn unparse_expr_dict(&mut self, node: &ExprDict<TextRange>) -> fmt::Result {
        let mut zipped = node.keys.iter().zip(node.values.iter()).peekable();

        self.write_str("{")?;
        while let Some((key, value)) = zipped.next() {
            match key {
                Some(key_value) => {
                    self.unparse_expr(key_value)?;
                    self.write_str(": ")?;
                }
                None => {
                    self.write_str("**")?;
                }
            }
            self.unparse_expr(value)?;
            if zipped.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        self.write_str("}")?;
        Ok(())
    }

    fn unparse_expr_set(&mut self, node: &ExprSet<TextRange>) -> fmt::Result {
        if node.elts.len() > 0 {
            self.write_str("{")?;
            let mut elts_iter = node.elts.iter().peekable();
            while let Some(expr) = elts_iter.next() {
                self.unparse_expr(expr)?;
                if elts_iter.peek().is_some() {
                    self.write_str(", ")?;
                }
            }
            self.write_str("}")?;
        } else {
            self.write_str("{*()}")?;
        }
        Ok(())
    }

    fn unparse_expr_list_comp(&mut self, node: &ExprListComp<TextRange>) -> fmt::Result {
        self.write_str("[")?;
        self.unparse_expr(&node.elt)?;
        for generator in &node.generators {
            self.unparse_comprehension(generator)?;
        }
        self.write_str("]")?;
        Ok(())
    }

    fn unparse_expr_set_comp(&mut self, node: &ExprSetComp<TextRange>) -> fmt::Result {
        self.write_str("{")?;
        self.unparse_expr(&node.elt)?;

        for generator in &node.generators {
            self.unparse_comprehension(generator)?;
        }
        self.write_str("}")?;
        Ok(())
    }

    fn unparse_expr_dict_comp(&mut self, node: &ExprDictComp<TextRange>) -> fmt::Result {
        self.write_str("{")?;
        self.unparse_expr(&node.key)?;
        self.write_str(": ")?;
        self.unparse_expr(&node.value)?;

        for generator in &node.generators {
            self.unparse_comprehension(generator)?;
        }
        Ok(())
    }

    fn unparse_expr_generator_exp(&mut self, node: &ExprGeneratorExp<TextRange>) -> fmt::Result {
        self.unparse_expr(&node.elt)?;

        for generator in &node.generators {
            self.unparse_comprehension(generator)?;
        }
        Ok(())
    }

    fn unparse_expr_await(&mut self, node: &ExprAwait<TextRange>) -> fmt::Result {
        let enum_member = Expr::Await(node.to_owned());
        self.delimit_precedence(&enum_member, |block_self| {
            block_self.write_str("await ")?;
            block_self.unparse_expr(&node.value)?;
            Ok(())
        })
    }

    fn unparse_expr_yield(&mut self, node: &ExprYield<TextRange>) -> fmt::Result {
        let enum_member = Expr::Yield(node.to_owned());
        self.delimit_precedence(&enum_member, |block_self| {
            block_self.write_str("yield")?;
            if let Some(expr) = &node.value {
                block_self.write_str(" ")?;
                block_self.unparse_expr(expr)?;
            }
            Ok(())
        })
    }

    fn unparse_expr_yield_from(&mut self, node: &ExprYieldFrom<TextRange>) -> fmt::Result {
        let enum_member = Expr::YieldFrom(node.to_owned());
        self.delimit_precedence(&enum_member, |block_self| {
            block_self.write_str("yield from ")?;

            block_self.unparse_expr(&node.value)?;
            Ok(())
        })
    }

    fn unparse_expr_compare(&mut self, node: &ExprCompare<TextRange>) -> fmt::Result {
        let enum_member = Expr::Compare(node.to_owned());
        let zipped = node.ops.iter().zip(node.comparators.iter());
        self.delimit_precedence(&enum_member, |block_self| {
            block_self.unparse_expr(&node.left)?;
            for (op, comp) in zipped {
                let operator = match op {
                    CmpOp::Eq => " == ",
                    CmpOp::Gt => " > ",
                    CmpOp::GtE => " >= ",
                    CmpOp::In => " in ",
                    CmpOp::Is => " is ",
                    CmpOp::IsNot => " is not ",
                    CmpOp::Lt => " < ",
                    CmpOp::LtE => " <= ",
                    CmpOp::NotEq => " != ",
                    CmpOp::NotIn => " not in ",
                };
                block_self.write_str(&operator)?;
                block_self.unparse_expr(comp)?;
            }
            Ok(())
        })
    }

    fn unparse_expr_call(&mut self, node: &ExprCall<TextRange>) -> fmt::Result {
        self.unparse_expr(&node.func)?;
        let mut args_iter = node.args.iter().peekable();
        let mut keywords_iter = node.keywords.iter().peekable();
        while let Some(arg) = args_iter.next() {
            self.unparse_expr(arg)?;
            if args_iter.peek().is_some() || keywords_iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        while let Some(keyword) = keywords_iter.next() {
            self.unparse_keyword(keyword)?;
            if keywords_iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        Ok(())
    }

    fn unparse_expr_formatted_value(
        &mut self,
        node: &ExprFormattedValue<TextRange>,
    ) -> fmt::Result {
        // TODO
        self.write_str("{ ")?;
        self.unparse_expr(&node.value)?;

        if let Some(format_spec) = &node.format_spec {
            self.write_str(":")?;
            self.unparse_expr(format_spec)?;
        }
        self.write_str(" }")?;
        Ok(())
    }

    fn unparse_expr_joined_str(&mut self, _node: &ExprJoinedStr<TextRange>) -> fmt::Result {
        // TODO
        Ok(())
    }

    fn _unparse_constant(&mut self, constant: &Constant) -> fmt::Result {
        return match constant {
            Constant::Tuple(values) => {
                self.write_str("(")?;
                let mut values_iter = values.iter().peekable();
                while let Some(value) = values_iter.next() {
                    self._unparse_constant(value)?;
                    if values_iter.peek().is_some() {
                        self.write_str(", ")?;
                    }
                }
                self.write_str(")")?;
                Ok(())
            }
            Constant::Ellipsis => self.write_str("..."),
            Constant::Bool(value) => {
                if *value {
                    self.write_str("True")
                } else {
                    self.write_str("False")
                }
            }
            Constant::Bytes(value) => {
                let utf8 = String::from_utf8(value.to_owned());

                match utf8 {
                    Ok(str_) => self.write_str(&str_),
                    Err(_err) => Err(fmt::Error),
                }
            }
            Constant::Int(value) => self.write_str(&value.to_string()),
            Constant::Str(value) => self.write_str(value),
            Constant::None => self.write_str("None"),
            Constant::Complex { real, imag: _ } => self.write_str(&real.to_string()),
            Constant::Float(value) => self.write_str(&value.to_string()),
        };
    }

    fn unparse_expr_constant(&mut self, node: &ExprConstant<TextRange>) -> fmt::Result {
        if node.kind.as_deref().is_some_and(|kind| kind == "u") {
            self.write_str("u")?;
        }
        self._unparse_constant(&node.value)
    }

    fn unparse_expr_attribute(&mut self, node: &ExprAttribute<TextRange>) -> fmt::Result {
        self.write_str(".")?;
        self.unparse_expr(&node.value)
    }
    fn unparse_expr_subscript(&mut self, node: &ExprSubscript<TextRange>) -> fmt::Result {
        self.unparse_expr(&node.value)?;
        self.write_str("[")?;
        self.unparse_expr(&node.slice)?;
        self.write_str("]")?;
        Ok(())
    }
    fn unparse_expr_starred(&mut self, node: &ExprStarred<TextRange>) -> fmt::Result {
        self.write_str("*")?;
        self.unparse_expr(&node.value)
    }

    fn unparse_expr_name(&mut self, node: &ExprName<TextRange>) -> fmt::Result {
        self.write_str(&node.id.as_str())
    }
    fn unparse_expr_list(&mut self, node: &ExprList<TextRange>) -> fmt::Result {
        let mut elts_iter = node.elts.iter().peekable();
        self.write_str("[")?;
        while let Some(expr) = elts_iter.next() {
            self.unparse_expr(expr)?;
        }
        self.write_str("]")?;
        Ok(())
    }

    fn unparse_expr_tuple(&mut self, node: &ExprTuple<TextRange>) -> fmt::Result {
        let mut elts_iter = node.elts.iter().peekable();
        self.write_str("(")?;
        while let Some(expr) = elts_iter.next() {
            self.unparse_expr(expr)?;
        }
        self.write_str(")")?;
        Ok(())
    }

    fn unparse_expr_slice(&mut self, node: &ExprSlice<TextRange>) -> fmt::Result {
        if let Some(lower) = &node.lower {
            self.unparse_expr(lower)?;
        }
        self.write_str(":")?;
        if let Some(upper) = &node.upper {
            self.unparse_expr(upper)?;
        }
        if let Some(step) = &node.step {
            self.write_str(":")?;
            self.unparse_expr(step)?;
        }
        Ok(())
    }

    fn unparse_operator(&mut self, node: &Operator) -> fmt::Result {
        self.write_str(match node {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::BitOr => "|",
            Operator::BitAnd => "&",
            Operator::BitXor => "^",
            Operator::Div => "/",
            Operator::FloorDiv => "//",
            Operator::LShift => "<<",
            Operator::MatMult => "@",
            Operator::Mod => "%",
            Operator::Pow => "**",
            Operator::RShift => ">>",
            Operator::Mult => "*",
        })
    }

    fn unparse_comprehension(&mut self, node: &Comprehension<TextRange>) -> fmt::Result {
        if node.is_async {
            self.write_str("async for")?;
        } else {
            self.write_str("for")?;
        }
        self.unparse_expr(&node.target)?;
        self.write_str(" in ")?;
        self.unparse_expr(&node.iter)?;
        for if_ in &node.ifs {
            self.unparse_expr(if_)?;
        }
        Ok(())
    }

    fn unparse_excepthandler(&mut self, node: &ExceptHandler<TextRange>) -> fmt::Result {
        match node {
            ExceptHandler::ExceptHandler(data) => self.unparse_excepthandler_except_handler(data),
        }
    }

    fn unparse_excepthandler_except_handler(
        &mut self,
        node: &ExceptHandlerExceptHandler<TextRange>,
    ) -> fmt::Result {
        if let Some(type_) = &node.type_ {
            self.unparse_expr(type_)?;
        }
        for stmt in &node.body {
            self.unparse_stmt(stmt)?;
        }
        Ok(())
    }

    fn unparse_arguments(&mut self, node: &Arguments<TextRange>) -> fmt::Result {
        let mut posonly_iter = node.posonlyargs.iter().peekable();
        let mut args_iter = node.args.iter().peekable();
        let mut kw_iter = node.kwonlyargs.iter().peekable();
        while let Some(posonly) = posonly_iter.next() {
            self.unparse_arg(posonly.as_arg())?;
            if let Some(default) = &posonly.default {
                self.write_str("=")?;
                self.unparse_expr(default)?;
            }

            if posonly_iter.peek().is_some() {
                self.write_str(", ")?;
            } else if args_iter.peek().is_some()
                || node.vararg.is_some()
                || node.vararg.is_some()
                || kw_iter.peek().is_some()
                || node.kwarg.is_some()
            {
                self.write_str(", /")?;
            }
        }

        while let Some(arg) = args_iter.next() {
            self.unparse_arg(arg.as_arg())?;
            if let Some(default) = &arg.default {
                self.write_str("=")?;
                self.unparse_expr(default)?;
            }
            if args_iter.peek().is_some()
                || node.vararg.is_some()
                || kw_iter.peek().is_some()
                || node.kwarg.is_some()
            {
                self.write_str(", ")?;
            }
        }

        if let Some(vararg) = &node.vararg {
            self.write_str("*")?;
            self.write_str(&vararg.arg)?;
            if let Some(annotation) = &vararg.annotation {
                self.write_str(": ")?;
                self.unparse_expr(annotation)?;
            }
            if kw_iter.peek().is_some() || node.kwarg.is_some() {
                self.write_str(", ")?;
            }
        }

        while let Some(kw) = kw_iter.next() {
            self.unparse_arg(kw.as_arg())?;
            if let Some(default) = &kw.default {
                self.write_str("=")?;
                self.unparse_expr(default)?;
            }
            if kw_iter.peek().is_some() || node.kwarg.is_some() {
                self.write_str(", ")?;
            }
        }

        if let Some(kwarg) = &node.kwarg {
            self.write_str("**")?;
            self.write_str(&kwarg.arg)?;
            if let Some(annotation) = &kwarg.annotation {
                self.write_str(": ")?;
                self.unparse_expr(&annotation)?;
            }
        }

        Ok(())
    }

    fn unparse_arg(&mut self, node: &Arg<TextRange>) -> fmt::Result {
        self.write_str(node.arg.as_str())?;
        if let Some(annotation) = &node.annotation {
            self.write_str(": ")?;
            self.unparse_expr(annotation)?;
        }
        Ok(())
    }

    fn unparse_keyword(&mut self, node: &Keyword<TextRange>) -> fmt::Result {
        if let Some(arg) = &node.arg {
            self.write_str(arg.as_str())?;
            self.write_str("=")?;
        } else {
            self.write_str("**")?;
        }

        self.unparse_expr(&node.value)?;
        Ok(())
    }

    fn unparse_alias(&mut self, node: &Alias<TextRange>) -> fmt::Result {
        self.write_str(node.name.as_str())?;
        if node.asname.is_some() {
            self.write_str(&format!(" as {}", node.asname.as_ref().unwrap()))?;
        }
        Ok(())
    }

    fn unparse_withitem(&mut self, node: &WithItem<TextRange>) -> fmt::Result {
        self.unparse_expr(&node.context_expr)?;
        if let Some(var) = &node.optional_vars {
            self.write_str(" as ")?;
            self.unparse_expr(var)?;
        }
        Ok(())
    }

    fn unparse_match_case(&mut self, node: &MatchCase<TextRange>) -> fmt::Result {
        self.fill("case ")?;
        self.unparse_pattern(&node.pattern)?;
        if let Some(guard) = &node.guard {
            self.write_str(" if ")?;
            self.unparse_expr(&guard)?;
        }
        self.write_str(":")?;
        self.block(|block_self| {
            for stmt in &node.body {
                block_self.unparse_stmt(stmt)?;
            }
            Ok(())
        })?;
        Ok(())
    }

    fn unparse_pattern(&mut self, node: &Pattern<TextRange>) -> fmt::Result {
        match node {
            Pattern::MatchValue(data) => self.unparse_pattern_match_value(data),
            Pattern::MatchSingleton(data) => self.unparse_pattern_match_singleton(data),
            Pattern::MatchSequence(data) => self.unparse_pattern_match_sequence(data),
            Pattern::MatchMapping(data) => self.unparse_pattern_match_mapping(data),
            Pattern::MatchClass(data) => self.unparse_pattern_match_class(data),
            Pattern::MatchStar(data) => self.unparse_pattern_match_star(data),
            Pattern::MatchAs(data) => self.unparse_pattern_match_as(data),
            Pattern::MatchOr(data) => self.unparse_pattern_match_or(data),
        }
    }

    fn unparse_pattern_match_value(&mut self, node: &PatternMatchValue<TextRange>) -> fmt::Result {
        self.unparse_expr(&node.value)
    }

    fn unparse_pattern_match_singleton(
        &mut self,
        node: &PatternMatchSingleton<TextRange>,
    ) -> fmt::Result {
        self.write_str(node.value.as_str().unwrap())
    }

    fn unparse_pattern_match_sequence(
        &mut self,
        node: &PatternMatchSequence<TextRange>,
    ) -> fmt::Result {
        let mut patterns_iter = node.patterns.iter().peekable();
        self.write_str("[")?;
        while let Some(pattern) = patterns_iter.next() {
            self.unparse_pattern(pattern)?;
            if patterns_iter.peek().is_some() {
                self.write_str(" , ")?;
            }
        }
        self.write_str("]")?;
        Ok(())
    }

    fn unparse_pattern_match_mapping(
        &mut self,
        node: &PatternMatchMapping<TextRange>,
    ) -> fmt::Result {
        let mut pairs_iter = node.keys.iter().zip(node.patterns.iter()).peekable();
        self.write_str("{")?;
        while let Some((key, pattern)) = pairs_iter.next() {
            self.unparse_expr(key)?;
            self.write_str(": ")?;
            self.unparse_pattern(pattern)?;
            if pairs_iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        if let Some(rest) = &node.rest {
            if node.keys.len() > 0 {
                self.write_str(", ")?;
            }
            self.write_str("**")?;
            self.write_str(rest.as_str())?;
        }

        self.write_str("}")?;
        Ok(())
    }

    fn unparse_pattern_match_class(&mut self, node: &PatternMatchClass<TextRange>) -> fmt::Result {
        let mut patterns_iter = node.patterns.iter().peekable();
        let mut kwd_iter = node
            .kwd_attrs
            .iter()
            .zip(node.kwd_patterns.iter())
            .peekable();
        self.unparse_expr(&node.cls)?;
        self.write_str("(")?;
        while let Some(pattern) = patterns_iter.next() {
            self.unparse_pattern(pattern)?;
            if patterns_iter.peek().is_some() || kwd_iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }
        while let Some((attr, pattern)) = kwd_iter.next() {
            self.write_str(attr.as_str())?;
            self.write_str("=")?;
            self.unparse_pattern(pattern)?;
            if kwd_iter.peek().is_some() {
                self.write_str(", ")?;
            }
        }

        self.write_str(")")?;
        Ok(())
    }

    fn unparse_pattern_match_star(&mut self, node: &PatternMatchStar<TextRange>) -> fmt::Result {
        let name = match &node.name {
            Some(name) => name.as_str(),
            None => "_",
        };
        self.write_str("*")?;
        self.write_str(name)?;
        Ok(())
    }

    fn unparse_pattern_match_as(&mut self, _node: &PatternMatchAs<TextRange>) -> fmt::Result {
        // TODO
        Ok(())
    }

    fn unparse_pattern_match_or(&mut self, node: &PatternMatchOr<TextRange>) -> fmt::Result {
        let mut patterns_iter = node.patterns.iter().peekable();
        while let Some(pattern) = patterns_iter.next() {
            self.unparse_pattern(pattern)?;
            if patterns_iter.peek().is_some() {
                self.write_str(" | ")?;
            }
        }
        Ok(())
    }

    fn unparse_type_param(&mut self, node: &TypeParam<TextRange>) -> fmt::Result {
        match node {
            TypeParam::TypeVar(data) => self.unparse_type_param_type_var(data),
            TypeParam::ParamSpec(data) => self.unparse_type_param_param_spec(data),
            TypeParam::TypeVarTuple(data) => self.unparse_type_param_type_var_tuple(data),
        }
    }

    fn unparse_type_param_type_var(&mut self, node: &TypeParamTypeVar<TextRange>) -> fmt::Result {
        self.write_str(&node.name)?;
        if let Some(bound) = &node.bound {
            self.write_str(": ")?;
            self.unparse_expr(bound)?;
        }
        Ok(())
    }

    fn unparse_type_param_param_spec(
        &mut self,
        node: &TypeParamParamSpec<TextRange>,
    ) -> fmt::Result {
        self.write_str("**")?;
        self.write_str(&node.name)?;
        Ok(())
    }

    fn unparse_type_param_type_var_tuple(
        &mut self,
        node: &TypeParamTypeVarTuple<TextRange>,
    ) -> fmt::Result {
        self.write_str("*")?;
        self.write_str(&node.name)?;
        Ok(())
    }
}
