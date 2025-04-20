use clippy_utils::diagnostics;
use rustc_hir::{Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// Checks whether a global variable defined.
    ///
    /// ### Why restrict this?
    ///
    /// - Global variables can be modified from any part of the program, making it difficult to
    ///   track and control their state.
    /// - Global variables introduce implicit dependencies that are not visible in function
    ///   signatures, making the code harder to understand and maintain.
    /// - Global variables introduce persistent state, complicating unit tests and making them
    ///   prone to side effects.
    /// - Global variables create tight coupling between different parts of the program, making it
    ///   harder to modify one part without affecting others.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// use std::sync::Mutex;
    ///
    /// struct State {}
    ///
    /// static STATE: Mutex<State> = Mutex::new(State {});
    ///
    /// fn foo() {
    ///    // Access global variable `STATE`.
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```no_run
    /// struct State {}
    ///
    /// fn foo(state: &mut State) {
    ///    // Access `state` argument instead of a global variable.
    /// }
    /// ```
    #[clippy::version = "1.88.0"]
    pub GLOBAL_VARIABLES,
    nursery,
    "global variables are discouraged"
}

declare_lint_pass!(GlobalVariables => [GLOBAL_VARIABLES]);

impl<'tcx> LateLintPass<'tcx> for GlobalVariables {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if matches!(item.kind, ItemKind::Static(..)) {
            let tcx = cx.tcx;

            if !tcx
                .type_of(item.owner_id.def_id)
                .skip_binder()
                .is_freeze(tcx, cx.typing_env())
            {
                diagnostics::span_lint(cx, GLOBAL_VARIABLES, item.span, "found global variable");
            }
        }
    }
}
