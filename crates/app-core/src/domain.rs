//! The order domain.
//!
//! # Why `domain.rs` + `domain/`, and not `domain/mod.rs`
//!
//! Since the 2018 edition, "the restriction that a module with submodules must
//! be named `mod.rs` is lifted. `foo.rs` can just be `foo.rs`, and the
//! submodule is still `foo/bar.rs`."
//! — <https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html>
//!
//! Both forms still work, so this is a style choice, but a one-sided one: with
//! `mod.rs` a project of any size ends up with eight editor tabs all labelled
//! `mod.rs`. Pick one form and use it everywhere; mixing them is the only
//! genuinely bad option.
//!
//! # Why this module re-exports
//!
//! Same reason as `lib.rs`: `order` and `customer` are private, so the split
//! between them is an implementation detail. The names below are the API.

mod customer;
mod order;

pub use customer::{Customer, CustomerId};
pub use order::{Order, OrderId, OrderLine, OrderStatus};
