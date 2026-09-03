//! Integration test: the order lifecycle, seen from outside the crate.
//!
//! # Why this file is here and not in `src/`
//!
//! "Each file in the `tests` directory is a separate crate, so we need to bring
//! our library into each test crate's scope. [...] They use your library in the
//! same way any other code would, which means they can only call functions that
//! are part of your library's public API."
//! — <https://doc.rust-lang.org/book/ch11-03-test-organization.html>
//!
//! That constraint is the whole value. Everything below compiles only because
//! `app_core`'s `lib.rs` re-exports it. If a refactor accidentally makes
//! `Order` unreachable from the crate root, this file fails to compile and the
//! unit tests in `src/domain/order.rs` still pass — which is exactly the signal
//! you want.
//!
//! Try it: uncomment the line marked "DOES NOT COMPILE" below.
//!
//! # Why the file name has a hyphen
//!
//! The target name is the file name, and target names (binaries, examples,
//! benches, integration tests) are kebab-case by convention. Rust *modules* are
//! `snake_case` (RFC 430) — but this file is not a module of anything, it is a
//! crate root. `cargo test --test order-lifecycle` is how you run just this file.
//!
//! Shared helpers, if this suite ever needs them, go in `tests/common/mod.rs`.
//! Files in subdirectories of `tests/` are not compiled as separate test crates.

#![allow(clippy::unwrap_used)]

use app_core::{Config, CustomerId, Error, Order, OrderId, OrderLine, OrderStatus};

fn placed_order() -> Order {
    let mut order = Order::new(OrderId(1), CustomerId(7));
    order
        .push_line(OrderLine::new("widget", 2, 1_500).unwrap())
        .unwrap();
    order.place(&Config::default()).unwrap();
    order
}

#[test]
fn an_order_can_be_drafted_placed_and_shipped() {
    let mut order = Order::new(OrderId(1), CustomerId(7));
    assert_eq!(order.status(), OrderStatus::Draft);

    order
        .push_line(OrderLine::new("widget", 2, 1_500).unwrap())
        .unwrap();
    order
        .push_line(OrderLine::new("gadget", 1, 999).unwrap())
        .unwrap();

    order.place(&Config::default()).unwrap();
    assert_eq!(order.status(), OrderStatus::Placed);

    order.ship().unwrap();
    assert_eq!(order.status(), OrderStatus::Shipped);
}

#[test]
fn the_total_is_the_sum_of_the_lines() {
    let order = placed_order();

    assert_eq!(order.total_cents(), 3_000);
    assert_eq!(order.lines().len(), 1);
}

#[test]
fn a_draft_order_can_be_cancelled() {
    let mut order = Order::new(OrderId(2), CustomerId(7));

    order.cancel().unwrap();

    assert_eq!(order.status(), OrderStatus::Cancelled);
}

#[test]
fn errors_are_matchable_by_downstream_code() {
    let mut order = Order::new(OrderId(3), CustomerId(7));

    // This is the reason a library uses `thiserror` and not `anyhow`: a caller
    // can branch on the specific failure. With `anyhow::Error` the only options
    // are string matching and downcasting.
    match order.place(&Config::default()) {
        Err(Error::EmptyOrder) => {}
        // `Error` is `#[non_exhaustive]`, so this arm is mandatory for us as an
        // external crate — and it is why adding a variant upstream is only a
        // minor version bump.
        other => panic!("expected EmptyOrder, got {other:?}"),
    }
}

#[test]
fn private_state_is_unreachable_from_out_here() {
    let order = placed_order();

    // DOES NOT COMPILE — `status` and `lines` are private fields:
    //     assert_eq!(order.status, OrderStatus::Placed);
    //     assert!(order.lines.is_empty());
    //
    // From outside the crate the accessors are all there is. That is not a
    // limitation to work around; it is the guarantee that lets `app-core`
    // rename or restructure those fields without breaking anyone.
    assert_eq!(order.status(), OrderStatus::Placed);
}
