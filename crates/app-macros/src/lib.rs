//! Procedural macros for the example application.
//!
//! # Why this is a separate crate
//!
//! Because it has to be. Setting `proc-macro = true` in `Cargo.toml` changes
//! how the crate is compiled: it is built for the *host* and dynamically loaded
//! into `rustc`, and it may export nothing but macros. Domain types cannot live
//! alongside them.
//!
//! This is worth stating plainly because it is the exception. Every other crate
//! boundary in this repo is a judgement call that could reasonably have gone
//! the other way. This one is not a judgement call at all.
//!
//! The usual way to hide the split from users is for the library crate to
//! re-export the macro (`pub use app_macros::shout;`), the way `serde`
//! re-exports `serde_derive` behind its `derive` feature. Then downstream code
//! depends on one crate and never learns there are two.
//!
//! # Why there is no `syn` or `quote` here
//!
//! Real macros use [`syn`] to parse Rust syntax, [`quote`] to build output, and
//! [`proc-macro2`] to make both testable outside the compiler. They are the
//! ecosystem default and you should reach for them.
//!
//! This crate parses one string literal, which is about the only thing you can
//! do with the raw [`proc_macro`] API without it becoming painful. Keeping it
//! dependency-free keeps this repository's dependency tree honest — the point
//! here is the *layout*, not the macro.
//!
//! [`syn`]: https://docs.rs/syn
//! [`quote`]: https://docs.rs/quote
//! [`proc-macro2`]: https://docs.rs/proc-macro2

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// Uppercases a string literal at compile time.
///
/// # Examples
///
/// ```
/// const BANNER: &str = app_macros::shout!("hello");
/// assert_eq!(BANNER, "HELLO");
/// ```
///
/// A non-literal argument is a compile error:
///
/// ```compile_fail
/// let name = "hello";
/// let _ = app_macros::shout!(name);
/// ```
#[proc_macro]
pub fn shout(input: TokenStream) -> TokenStream {
    match shout_impl(input) {
        Ok(stream) => stream,
        Err(message) => compile_error(&message),
    }
}

/// The real body, split out so the error path is one `match` instead of five
/// early returns building token streams by hand.
fn shout_impl(input: TokenStream) -> Result<TokenStream, String> {
    let mut tokens = input.into_iter();

    let Some(TokenTree::Literal(literal)) = tokens.next() else {
        return Err("shout! expects a single string literal".to_owned());
    };
    if tokens.next().is_some() {
        return Err("shout! expects exactly one argument".to_owned());
    }

    let rendered = literal.to_string();
    let contents = unquote(&rendered)
        .ok_or_else(|| format!("shout! expects a plain string literal, got `{rendered}`"))?;

    Ok(TokenStream::from(TokenTree::Literal(Literal::string(
        &contents.to_uppercase(),
    ))))
}

/// Strips the surrounding quotes from a rendered string literal.
///
/// Returns `None` for anything that is not a plain `"..."` literal — numbers,
/// byte strings, raw strings. A real implementation would hand this to
/// `syn::LitStr`, which also resolves escape sequences; this one deliberately
/// refuses input it cannot handle correctly rather than mangling it.
fn unquote(rendered: &str) -> Option<&str> {
    let inner = rendered.strip_prefix('"')?.strip_suffix('"')?;
    // An escape sequence would need real unescaping, which is exactly the job
    // this crate is not equipped to do.
    if inner.contains('\\') {
        return None;
    }
    Some(inner)
}

/// Builds `compile_error!("...")` as a token stream.
///
/// Emitting `compile_error!` rather than panicking gives the user a diagnostic
/// pointing at their code instead of an internal compiler panic message.
fn compile_error(message: &str) -> TokenStream {
    TokenStream::from_iter([
        TokenTree::Ident(Ident::new("compile_error", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from(TokenTree::Literal(Literal::string(message))),
        )),
    ])
}

#[cfg(test)]
mod tests {
    //! Unit tests in a proc-macro crate can only cover the *pure* helpers.
    //!
    //! `proc_macro::TokenStream` panics when constructed outside a real
    //! compilation, so `shout_impl` cannot be called from here at all. That is
    //! the practical reason real macro crates parse into `proc_macro2` types:
    //! those work anywhere, which makes the interesting logic unit-testable.
    //!
    //! Everything else is tested from `tests/shout.rs`, which is a separate
    //! crate and can therefore invoke the macro for real.

    use super::unquote;

    #[test]
    fn plain_literals_are_unquoted() {
        assert_eq!(unquote(r#""hello""#), Some("hello"));
        assert_eq!(unquote(r#""""#), Some(""));
    }

    #[test]
    fn anything_else_is_rejected() {
        assert_eq!(unquote("42"), None);
        assert_eq!(unquote(r#"b"bytes""#), None);
        assert_eq!(unquote(r#""with \n escape""#), None);
    }
}
