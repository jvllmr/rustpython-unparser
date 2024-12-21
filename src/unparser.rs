use std::fmt;
use std::io::Write;

use rustpython_ast::{
    text_size::TextRange, Alias, Arg, Arguments, BoolOp, CmpOp, Comprehension, ExceptHandler,
    ExceptHandlerExceptHandler, Expr, ExprAttribute, ExprAwait, ExprBinOp, ExprBoolOp, ExprCall,
    ExprCompare, ExprConstant, ExprContext, ExprDict, ExprDictComp, ExprFormattedValue,
    ExprGeneratorExp, ExprIfExp, ExprJoinedStr, ExprLambda, ExprList, ExprListComp, ExprName,
    ExprNamedExpr, ExprSet, ExprSetComp, ExprSlice, ExprStarred, ExprSubscript, ExprTuple,
    ExprUnaryOp, ExprYield, ExprYieldFrom, Keyword, MatchCase, Operator, Pattern, PatternMatchAs,
    PatternMatchClass, PatternMatchMapping, PatternMatchOr, PatternMatchSequence,
    PatternMatchSingleton, PatternMatchStar, PatternMatchValue, Stmt, StmtAnnAssign, StmtAssert,
    StmtAssign, StmtAsyncFor, StmtAsyncFunctionDef, StmtAsyncWith, StmtAugAssign, StmtBreak,
    StmtClassDef, StmtContinue, StmtDelete, StmtExpr, StmtFor, StmtFunctionDef, StmtGlobal, StmtIf,
    StmtImport, StmtImportFrom, StmtMatch, StmtNonlocal, StmtPass, StmtRaise, StmtReturn, StmtTry,
    StmtTryStar, StmtTypeAlias, StmtWhile, StmtWith, TypeParam, TypeParamParamSpec,
    TypeParamTypeVar, TypeParamTypeVarTuple, UnaryOp, WithItem,
};

pub trait Unparser<'a, W = fmt::Formatter<'a>, R = TextRange> {
    fn unparse_stmt(&mut self, node: Stmt<R>) {
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

    fn unparse_stmt_pass(&mut self, node: StmtPass<R>) {}

    fn unparse_stmt_break(&mut self, node: StmtBreak<R>) {}

    fn unparse_stmt_continue(&mut self, node: StmtContinue<R>) {}

    fn unparse_stmt_function_def(&mut self, node: StmtFunctionDef<R>) {
        {
            let value = node.args;
            self.unparse_arguments(*value);
        }
        for value in node.body {
            self.unparse_stmt(value);
        }
        for value in node.decorator_list {
            self.unparse_expr(value);
        }
        if let Some(value) = node.returns {
            self.unparse_expr(*value);
        }
        for value in node.type_params {
            self.unparse_type_param(value);
        }
    }

    fn unparse_stmt_async_function_def(&mut self, node: StmtAsyncFunctionDef<R>) {
        {
            let value = node.args;
            self.unparse_arguments(*value);
        }
        for value in node.body {
            self.unparse_stmt(value);
        }
        for value in node.decorator_list {
            self.unparse_expr(value);
        }
        if let Some(value) = node.returns {
            self.unparse_expr(*value);
        }
        for value in node.type_params {
            self.unparse_type_param(value);
        }
    }

    fn unparse_stmt_class_def(&mut self, node: StmtClassDef<R>) {
        for value in node.bases {
            self.unparse_expr(value);
        }
        for value in node.keywords {
            self.unparse_keyword(value);
        }
        for value in node.body {
            self.unparse_stmt(value);
        }
        for value in node.decorator_list {
            self.unparse_expr(value);
        }
        for value in node.type_params {
            self.unparse_type_param(value);
        }
    }

    fn unparse_stmt_return(&mut self, node: StmtReturn<R>) {
        if let Some(value) = node.value {
            self.unparse_expr(*value);
        }
    }
    fn unparse_stmt_delete(&mut self, node: StmtDelete<R>) {
        for value in node.targets {
            self.unparse_expr(value);
        }
    }

    fn unparse_stmt_assign(&mut self, node: StmtAssign<R>) {
        for value in node.targets {
            self.unparse_expr(value);
        }
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
    }

    fn unparse_stmt_type_alias(&mut self, node: StmtTypeAlias<R>) {
        {
            let value = node.name;
            self.unparse_expr(*value);
        }
        for value in node.type_params {
            self.unparse_type_param(value);
        }
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
    }

    fn unparse_stmt_aug_assign(&mut self, node: StmtAugAssign<R>) {
        {
            let value = node.target;
            self.unparse_expr(*value);
        }
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
    }

    fn unparse_stmt_ann_assign(&mut self, node: StmtAnnAssign<R>) {
        {
            let value = node.target;
            self.unparse_expr(*value);
        }
        {
            let value = node.annotation;
            self.unparse_expr(*value);
        }
        if let Some(value) = node.value {
            self.unparse_expr(*value);
        }
    }

    fn unparse_stmt_for(&mut self, node: StmtFor<R>) {
        {
            let value = node.target;
            self.unparse_expr(*value);
        }
        {
            let value = node.iter;
            self.unparse_expr(*value);
        }
        for value in node.body {
            self.unparse_stmt(value);
        }
        for value in node.orelse {
            self.unparse_stmt(value);
        }
    }
    fn unparse_stmt_async_for(&mut self, node: StmtAsyncFor<R>) {
        {
            let value = node.target;
            self.unparse_expr(*value);
        }
        {
            let value = node.iter;
            self.unparse_expr(*value);
        }
        for value in node.body {
            self.unparse_stmt(value);
        }
        for value in node.orelse {
            self.unparse_stmt(value);
        }
    }
    fn unparse_stmt_while(&mut self, node: StmtWhile<R>) {
        {
            let value = node.test;
            self.unparse_expr(*value);
        }
        for value in node.body {
            self.unparse_stmt(value);
        }
        for value in node.orelse {
            self.unparse_stmt(value);
        }
    }

    fn unparse_stmt_if(&mut self, node: StmtIf<R>) {
        {
            let value = node.test;
            self.unparse_expr(*value);
        }
        for value in node.body {
            self.unparse_stmt(value);
        }
        for value in node.orelse {
            self.unparse_stmt(value);
        }
    }

    fn unparse_stmt_with(&mut self, node: StmtWith<R>) {
        for value in node.items {
            self.unparse_withitem(value);
        }
        for value in node.body {
            self.unparse_stmt(value);
        }
    }
    fn unparse_stmt_async_with(&mut self, node: StmtAsyncWith<R>) {
        for value in node.items {
            self.unparse_withitem(value);
        }
        for value in node.body {
            self.unparse_stmt(value);
        }
    }

    fn unparse_stmt_match(&mut self, node: StmtMatch<R>) {
        {
            let value = node.subject;
            self.unparse_expr(*value);
        }
        for value in node.cases {
            self.unparse_match_case(value);
        }
    }

    fn unparse_stmt_raise(&mut self, node: StmtRaise<R>) {
        if let Some(value) = node.exc {
            self.unparse_expr(*value);
        }
        if let Some(value) = node.cause {
            self.unparse_expr(*value);
        }
    }

    fn unparse_stmt_try(&mut self, node: StmtTry<R>) {
        for value in node.body {
            self.unparse_stmt(value);
        }
        for value in node.handlers {
            self.unparse_excepthandler(value);
        }
        for value in node.orelse {
            self.unparse_stmt(value);
        }
        for value in node.finalbody {
            self.unparse_stmt(value);
        }
    }
    fn unparse_stmt_try_star(&mut self, node: StmtTryStar<R>) {
        for value in node.body {
            self.unparse_stmt(value);
        }
        for value in node.handlers {
            self.unparse_excepthandler(value);
        }
        for value in node.orelse {
            self.unparse_stmt(value);
        }
        for value in node.finalbody {
            self.unparse_stmt(value);
        }
    }
    fn unparse_stmt_assert(&mut self, node: StmtAssert<R>) {
        {
            let value = node.test;
            self.unparse_expr(*value);
        }
        if let Some(value) = node.msg {
            self.unparse_expr(*value);
        }
    }

    fn unparse_stmt_import(&mut self, node: StmtImport<R>) {
        for value in node.names {
            self.unparse_alias(value);
        }
    }
    fn unparse_stmt_import_from(&mut self, node: StmtImportFrom<R>) {
        for value in node.names {
            self.unparse_alias(value);
        }
    }
    fn unparse_stmt_global(&mut self, node: StmtGlobal<R>) {}
    fn unparse_stmt_nonlocal(&mut self, node: StmtNonlocal<R>) {}
    fn unparse_stmt_expr(&mut self, node: StmtExpr<R>) {
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
    }

    fn unparse_expr(&mut self, node: Expr<R>) {
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

    fn unparse_expr_bool_op(&mut self, node: ExprBoolOp<R>) {
        for value in node.values {
            self.unparse_expr(value);
        }
    }

    fn unparse_expr_named_expr(&mut self, node: ExprNamedExpr<R>) {
        {
            let value = node.target;
            self.unparse_expr(*value);
        }
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
    }

    fn unparse_expr_bin_op(&mut self, node: ExprBinOp<R>) {
        {
            let value = node.left;
            self.unparse_expr(*value);
        }
        {
            let value = node.right;
            self.unparse_expr(*value);
        }
    }

    fn unparse_expr_unary_op(&mut self, node: ExprUnaryOp<R>) {
        {
            let value = node.operand;
            self.unparse_expr(*value);
        }
    }
    fn unparse_expr_lambda(&mut self, node: ExprLambda<R>) {
        {
            let value = node.args;
            self.unparse_arguments(*value);
        }
        {
            let value = node.body;
            self.unparse_expr(*value);
        }
    }
    fn unparse_expr_if_exp(&mut self, node: ExprIfExp<R>) {
        {
            let value = node.test;
            self.unparse_expr(*value);
        }
        {
            let value = node.body;
            self.unparse_expr(*value);
        }
        {
            let value = node.orelse;
            self.unparse_expr(*value);
        }
    }

    fn unparse_expr_dict(&mut self, node: ExprDict<R>) {
        for value in node.keys.into_iter().flatten() {
            self.unparse_expr(value);
        }
        for value in node.values {
            self.unparse_expr(value);
        }
    }

    fn unparse_expr_set(&mut self, node: ExprSet<R>) {
        for value in node.elts {
            self.unparse_expr(value);
        }
    }

    fn unparse_expr_list_comp(&mut self, node: ExprListComp<R>) {
        {
            let value = node.elt;
            self.unparse_expr(*value);
        }
        for value in node.generators {
            self.unparse_comprehension(value);
        }
    }

    fn unparse_expr_set_comp(&mut self, node: ExprSetComp<R>) {
        {
            let value = node.elt;
            self.unparse_expr(*value);
        }
        for value in node.generators {
            self.unparse_comprehension(value);
        }
    }

    fn unparse_expr_dict_comp(&mut self, node: ExprDictComp<R>) {
        {
            let value = node.key;
            self.unparse_expr(*value);
        }
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
        for value in node.generators {
            self.unparse_comprehension(value);
        }
    }

    fn unparse_expr_generator_exp(&mut self, node: ExprGeneratorExp<R>) {
        {
            let value = node.elt;
            self.unparse_expr(*value);
        }
        for value in node.generators {
            self.unparse_comprehension(value);
        }
    }

    fn unparse_expr_await(&mut self, node: ExprAwait<R>) {
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
    }

    fn unparse_expr_yield(&mut self, node: ExprYield<R>) {
        if let Some(value) = node.value {
            self.unparse_expr(*value);
        }
    }

    fn unparse_expr_yield_from(&mut self, node: ExprYieldFrom<R>) {
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
    }

    fn unparse_expr_compare(&mut self, node: ExprCompare<R>) {
        {
            let value = node.left;
            self.unparse_expr(*value);
        }
        for value in node.comparators {
            self.unparse_expr(value);
        }
    }

    fn unparse_expr_call(&mut self, node: ExprCall<R>) {
        {
            let value = node.func;
            self.unparse_expr(*value);
        }
        for value in node.args {
            self.unparse_expr(value);
        }
        for value in node.keywords {
            self.unparse_keyword(value);
        }
    }

    fn unparse_expr_formatted_value(&mut self, node: ExprFormattedValue<R>) {
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
        if let Some(value) = node.format_spec {
            self.unparse_expr(*value);
        }
    }

    fn unparse_expr_joined_str(&mut self, node: ExprJoinedStr<R>) {
        for value in node.values {
            self.unparse_expr(value);
        }
    }
    fn unparse_expr_constant(&mut self, node: ExprConstant<R>) {}

    fn unparse_expr_attribute(&mut self, node: ExprAttribute<R>) {
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
    }
    fn unparse_expr_subscript(&mut self, node: ExprSubscript<R>) {
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
        {
            let value = node.slice;
            self.unparse_expr(*value);
        }
    }
    fn unparse_expr_starred(&mut self, node: ExprStarred<R>) {
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
    }

    fn unparse_expr_name(&mut self, node: ExprName<R>) {}
    fn unparse_expr_list(&mut self, node: ExprList<R>) {
        for value in node.elts {
            self.unparse_expr(value);
        }
    }

    fn unparse_expr_tuple(&mut self, node: ExprTuple<R>) {
        for value in node.elts {
            self.unparse_expr(value);
        }
    }

    fn unparse_expr_slice(&mut self, node: ExprSlice<R>) {
        if let Some(value) = node.lower {
            self.unparse_expr(*value);
        }
        if let Some(value) = node.upper {
            self.unparse_expr(*value);
        }
        if let Some(value) = node.step {
            self.unparse_expr(*value);
        }
    }
    fn unparse_expr_context(&mut self, node: ExprContext) {}

    fn unparse_boolop(&mut self, node: BoolOp) {}
    fn unparse_operator(&mut self, node: Operator) {}
    fn unparse_unaryop(&mut self, node: UnaryOp) {}
    fn unparse_cmpop(&mut self, node: CmpOp) {}
    fn unparse_comprehension(&mut self, node: Comprehension<R>) {}

    fn unparse_excepthandler(&mut self, node: ExceptHandler<R>) {
        match node {
            ExceptHandler::ExceptHandler(data) => self.unparse_excepthandler_except_handler(data),
        }
    }

    fn unparse_excepthandler_except_handler(&mut self, node: ExceptHandlerExceptHandler<R>) {
        if let Some(value) = node.type_ {
            self.unparse_expr(*value);
        }
        for value in node.body {
            self.unparse_stmt(value);
        }
    }

    fn unparse_arguments(&mut self, node: Arguments<R>) {}

    fn unparse_arg(&mut self, node: Arg<R>) {}

    fn unparse_keyword(&mut self, node: Keyword<R>) {}

    fn unparse_alias(&mut self, node: Alias<R>) {}

    fn unparse_withitem(&mut self, node: WithItem<R>) {}

    fn unparse_match_case(&mut self, node: MatchCase<R>) {}

    fn unparse_pattern(&mut self, node: Pattern<R>) {
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

    fn unparse_pattern_match_value(&mut self, node: PatternMatchValue<R>) {
        {
            let value = node.value;
            self.unparse_expr(*value);
        }
    }

    fn unparse_pattern_match_singleton(&mut self, node: PatternMatchSingleton<R>) {}

    fn unparse_pattern_match_sequence(&mut self, node: PatternMatchSequence<R>) {
        for value in node.patterns {
            self.unparse_pattern(value);
        }
    }

    fn unparse_pattern_match_mapping(&mut self, node: PatternMatchMapping<R>) {
        for value in node.keys {
            self.unparse_expr(value);
        }
        for value in node.patterns {
            self.unparse_pattern(value);
        }
    }

    fn unparse_pattern_match_class(&mut self, node: PatternMatchClass<R>) {
        {
            let value = node.cls;
            self.unparse_expr(*value);
        }
        for value in node.patterns {
            self.unparse_pattern(value);
        }
        for value in node.kwd_patterns {
            self.unparse_pattern(value);
        }
    }

    fn unparse_pattern_match_star(&mut self, node: PatternMatchStar<R>) {}

    fn unparse_pattern_match_as(&mut self, node: PatternMatchAs<R>) {
        if let Some(value) = node.pattern {
            self.unparse_pattern(*value);
        }
    }

    fn unparse_pattern_match_or(&mut self, node: PatternMatchOr<R>) {
        for value in node.patterns {
            self.unparse_pattern(value);
        }
    }

    fn unparse_type_param(&mut self, node: TypeParam<R>) {
        match node {
            TypeParam::TypeVar(data) => self.unparse_type_param_type_var(data),
            TypeParam::ParamSpec(data) => self.unparse_type_param_param_spec(data),
            TypeParam::TypeVarTuple(data) => self.unparse_type_param_type_var_tuple(data),
        }
    }

    fn unparse_type_param_type_var(&mut self, node: TypeParamTypeVar<R>) {
        if let Some(value) = node.bound {
            self.unparse_expr(*value);
        }
    }

    fn unparse_type_param_param_spec(&mut self, node: TypeParamParamSpec<R>) {}

    fn unparse_type_param_type_var_tuple(&mut self, node: TypeParamTypeVarTuple<R>) {}
}
