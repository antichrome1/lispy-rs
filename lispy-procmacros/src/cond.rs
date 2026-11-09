/// Various conditional constructs.
///
/// - Each clause contains a condition followed by a block.
/// - The first matching clause runs and exits the `cond!` block.
/// - A final `true` clause acts as the default branch.
/// - If no condition matches and there is no final `true` clause, the
///   expression evaluates to `()`.  Use of a default clause is
///   encouraged.
/// - Is useful when there are a lot of conditions you want to check
///   against (spamming `else if` obscures the code at that point).
///
/// # Example skeleton of cond
///
/// ```ignore
/// cond! {
///     cond1 { body1 }
///     cond2 { body2 }
///     true  { fallback }
/// }
/// ```
///
/// # Usage Example
///
/// ```
/// use lispy_procmacros::cond;
///
/// fn selects_first_matching_clause() {
///     let value = 2;
///
///     let result = cond! {
///         value == 1 {
///             "one"
///         }
///         value == 2 {
///             "two"
///         }
///         true {
///             "not 1 or 2"
///         }
///     };
///
///     assert_eq!(result, "two");
/// }
///
/// selects_first_matching_clause();
///
/// ```
#[proc_macro]
pub fn cond(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);

    // Split the input into (condition tokens, body group contents) pairs.
    let mut arms: Vec<(TokenStream2, TokenStream2)> = Vec::new();
    let mut pending_cond = TokenStream2::new();

    for tt in input {
        match tt {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                if pending_cond.is_empty() {
                    return error("cond!: a `{ ... }` body must be preceded by a condition");
                }
                arms.push((std::mem::take(&mut pending_cond), group.stream()));
            }
            other => pending_cond.extend(std::iter::once(other)),
        }
    }

    if !pending_cond.is_empty() {
        return error("cond!: condition without a following `{ ... }` body");
    }

    // Like Lisp's `nil` result for an empty cond.
    if arms.is_empty() {
        return quote! { () }.into();
    }

    let mut expanded = TokenStream2::new();

    for (i, (cond, body)) in arms.into_iter().enumerate() {
        let is_true = is_true_literal(&cond);

        if i == 0 {
            if is_true {
                // `cond! { true { ... } }` on its own is just a block.
                expanded.extend(quote! { { #body } });
            } else {
                expanded.extend(quote! { if #cond { #body } });
            }
        } else if is_true {
            expanded.extend(quote! { else { #body } });
        } else {
            expanded.extend(quote! { else if #cond { #body } });
        }
    }

    expanded.into()
}

/// True if the condition is exactly the literal `true`.
fn is_true_literal(cond: &TokenStream2) -> bool {
    let mut iter = cond.clone().into_iter();
    matches!(
        (iter.next(), iter.next()),
        (Some(TokenTree::Ident(ident)), None) if ident == "true"
    )
}

fn error(msg: &str) -> TokenStream {
    quote! { compile_error!(#msg); }.into()
}
