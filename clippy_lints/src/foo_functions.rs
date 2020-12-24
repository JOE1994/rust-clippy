use rustc_ast::{node_id::NodeId, visit::FnKind};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::Span;

use crate::utils::span_lint_and_help;

declare_clippy_lint! {
    /// **What it does:** warns to rename functions with name `foo`
    ///
    /// **Why is this bad?** `foo` is not a descriptive name..
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    pub FOO_FUNCTIONS,
    pedantic,
    "default lint description"
}

declare_lint_pass!(FooFunctions => [FOO_FUNCTIONS]);

impl EarlyLintPass for FooFunctions {
    fn check_fn(&mut self, cx: &EarlyContext<'_>, fn_kind: FnKind<'_>, span: Span, _: NodeId) {
        if is_foo_fn(fn_kind) {
            span_lint_and_help(
                cx,
                FOO_FUNCTIONS,
                span,
                "function named `foo`",
                None,
                "consider using a more meaningful name",
            );
        }
    }
}

fn is_foo_fn(fn_kind: FnKind<'_>) -> bool {
    match fn_kind {
        FnKind::Fn(_, ident, ..) => {
            // check if `fn` name is `foo`
            let mut x = ident.name.as_str().to_string();
            x.make_ascii_lowercase();

            x == "foo"
        },
        // ignore closures
        FnKind::Closure(..) => false,
    }
}
