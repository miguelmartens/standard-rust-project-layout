//! Integration test for the macro itself.
//!
//! A proc-macro crate cannot invoke its own macros from `src/` — the macro has
//! to be compiled and loaded before it can run. `tests/` is a separate crate
//! that depends on this one, so here the macro works exactly as it does for a
//! downstream user. Same reasoning as any other integration test, just with a
//! harder constraint behind it.

const GREETING: &str = app_macros::shout!("hello, world");

#[test]
fn a_literal_is_uppercased_at_compile_time() {
    assert_eq!(GREETING, "HELLO, WORLD");
}

#[test]
fn the_result_is_usable_in_const_context() {
    // The macro expands to a literal, so it works anywhere a literal does.
    const _ASSERTION: () = assert!(!GREETING.is_empty());
}

#[test]
fn non_ascii_is_uppercased_by_the_unicode_rules() {
    assert_eq!(app_macros::shout!("straße"), "STRASSE");
}
