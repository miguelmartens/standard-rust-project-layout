//! Domain logic for the example application.
//!
//! # Why this crate exists
//!
//! It is the stable core: types and rules, no I/O. It knows nothing about
//! `clap`, about stdout, or about how a configuration file reaches it. That is
//! what makes it testable without a process, and reusable from something other
//! than [`app-cli`] — a server, a WASM build, a fuzz target.
//!
//! That separation is the *only* justification for it being a separate crate.
//! "It felt tidier" is not one. See the README section "When *not* to split".
//!
//! # Why `lib.rs` looks like this
//!
//! `lib.rs` is a façade: it declares the modules and re-exports the public API.
//! Note that every `mod` below is **private**. Callers write
//! `app_core::Order`, never `app_core::domain::order::Order`.
//!
//! The payoff is that the internal module tree is not part of the public API.
//! Splitting `domain::order` into three modules tomorrow is an internal
//! refactor, not a breaking change, because no downstream `use` statement
//! mentions it.
//!
//! It is also why no directory in this repository means "private". Privacy is a
//! language feature the compiler enforces, per item and wherever the item sits
//! on disk; a directory name is a convention a linter has to enforce. See the
//! README section "Privacy is a language feature".
//!
//! [`app-cli`]: https://github.com/example/rust-project-layout
//!
//! # Examples
//!
//! ```
//! use app_core::{Config, CustomerId, Order, OrderId, OrderLine};
//!
//! let mut order = Order::new(OrderId(1), CustomerId(42));
//! order.push_line(OrderLine::new("widget", 3, 1_250)?)?;
//! order.place(&Config::default())?;
//!
//! assert_eq!(order.total_cents(), 3_750);
//! # Ok::<(), app_core::Error>(())
//! ```

mod config;
mod domain;
mod error;

pub use crate::config::Config;
pub use crate::domain::{Customer, CustomerId, Order, OrderId, OrderLine, OrderStatus};
pub use crate::error::{Error, Result};
