//! An end-to-end walk through the public API.
//!
//! # Why `examples/` and not `docs/` or a `README` snippet
//!
//! Cargo compiles everything in `examples/` as part of
//! `cargo build --examples`, and CI runs `--all-targets`. An example that stops
//! compiling breaks the build; a snippet in a README rots in silence.
//!
//! Run it with:
//!
//! ```text
//! cargo run --package app-core --example simple
//! ```
//!
//! The target name is the file stem, `simple`. A multi-file example would be
//! `examples/simple/main.rs` plus sibling modules, and would still be called
//! `simple`.

use app_core::{Config, Customer, CustomerId, Order, OrderId, OrderLine};

fn main() -> Result<(), app_core::Error> {
    let config = Config::default();
    let customer = Customer::new(CustomerId(42), "Ada Lovelace");

    let mut order = Order::new(OrderId(1), customer.id());
    order.push_line(OrderLine::new("widget", 3, 1_250)?)?;
    order.push_line(OrderLine::new("gadget", 1, 999)?)?;

    order.place(&config)?;

    println!("customer: {}", customer.name());
    println!("status:   {}", order.status());
    println!(
        "total:    {} {}.{:02}",
        config.currency,
        order.total_cents() / 100,
        order.total_cents() % 100
    );

    Ok(())
}
