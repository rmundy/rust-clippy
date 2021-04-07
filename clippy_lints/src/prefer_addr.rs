declare_clippy_lint! {
    /// **What it does:** Use `ptr::addr_of` or `ptr::addr_of_mut` when applicable
    ///
    /// **Why is this bad?** `&expr as *const T` or `&mut expr as *mut T` can be used to an unsafe reference
    /// that points to an uninitialized value or unaligned place
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// let val = 1;
    /// let p = &val as *const i32;

    /// let mut val_mut = 1;
    /// let p_mut = &mut val_mut as *mut i32;
    ///
    /// assert!(a as *const _ as usize == b as *const _ as usize);
    /// ```
    /// Use instead:
    /// ```rust
    /// let val = 1;
    /// let p = ptr::addr_of!(val);

    /// let mut val_mut = 1;
    /// let p_mut = ptr::addr_of_mut!(val_mut);
    /// ```
    ///
    ///
    pub PREFER_ADDR,
    style,
    "Use `ptr::addr_of` or `ptr::addr_of_mut` when applicable"
}

declare_lint_pass!(PreferAddr => [PREFER_ADDR]);

static LINT_MSG: &str = "Use `ptr::addr_of` or `ptr::addr_of_mut` when applicable";

impl LateLintPass<'_> for PreferAddr {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if in_macro(expr.span) {
            return;
        }
    }
}

