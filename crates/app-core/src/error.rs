//! The crate's error type.
//!
//! # Why a separate file
//!
//! One error type per crate, in one obvious place. The alternative — an error
//! enum per module — pushes conversion boilerplate onto every caller and makes
//! the crate's failure modes impossible to read in one sitting.
//!
//! # Why `thiserror` and not `anyhow`
//!
//! This is a library. Its callers need to *match* on failures to decide what to
//! do, so the error must be a real type with real variants. `thiserror`
//! generates the [`Display`] and [`std::error::Error`] impls and nothing else —
//! it disappears from the public API.
//!
//! `anyhow::Error` is the opposite trade: one opaque type, no matching,
//! excellent ergonomics. That is exactly right for a binary, which is why
//! [`app-cli`](../../app-cli/index.html) uses it and this crate does not.
//!
//! [`Display`]: std::fmt::Display

use crate::domain::OrderStatus;

/// Everything that can go wrong in this crate.
///
/// # Why `#[non_exhaustive]`
///
/// Adding a variant to a public enum is normally a breaking change: downstream
/// `match` statements stop compiling. `#[non_exhaustive]` forces external
/// callers to write a `_ => ...` arm up front, so new variants become a *minor*
/// version bump instead of a major one.
///
/// The cost is real — callers cannot prove they have handled everything — and
/// it is the right trade for an error enum, which grows over time by nature.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An order was submitted with no lines on it.
    #[error("an order must contain at least one line")]
    EmptyOrder,

    /// A line was created with a zero quantity.
    #[error("quantity for SKU `{sku}` must be greater than zero")]
    InvalidQuantity {
        /// The SKU of the offending line.
        sku: String,
    },

    /// An order has more lines than the configuration permits.
    #[error("order has {actual} lines, but the configured maximum is {max}")]
    TooManyLines {
        /// How many lines the order actually has.
        actual: usize,
        /// The configured limit.
        max: usize,
    },

    /// A state transition the order lifecycle does not allow.
    #[error("cannot move an order from {from} to {to}")]
    InvalidTransition {
        /// The status the order is currently in.
        from: OrderStatus,
        /// The status the caller asked for.
        to: OrderStatus,
    },
}

/// A `Result` that defaults to this crate's [`Error`].
///
/// The `E = Error` default is what makes `fn f() -> Result<T>` read well while
/// still allowing `Result<T, OtherError>` where a caller needs it.
pub type Result<T, E = Error> = core::result::Result<T, E>;
