//! A benchmark for `Order::total_cents`.
//!
//! # Why `harness = false` and a hand-rolled timing loop
//!
//! The stock libtest bench harness needs `#[bench]`, which is still unstable,
//! so a stable-only project cannot use it. The two real options are the
//! `criterion` and `divan` crates, both of which work by declaring
//! `harness = false` in `Cargo.toml` and providing their own `main`.
//!
//! This repo has no benchmarking dependency, so it does the same thing by hand.
//! **Do not copy this loop into a real project** — it has no warm-up, no
//! statistics, and no outlier detection. Copy the `harness = false` wiring, and
//! put criterion or divan behind it.
//!
//! Run it with:
//!
//! ```text
//! cargo bench --package app-core --bench order-total
//! ```
//!
//! `[profile.bench]` in the workspace root sets `debug = true`, so a profiler
//! attached to this binary can attribute samples to source lines.

use std::hint::black_box;
use std::time::Instant;

use app_core::{CustomerId, Order, OrderId, OrderLine};

const LINES: u32 = 64;
const ITERATIONS: u32 = 10_000;

fn main() {
    let order = build_order();

    let start = Instant::now();
    let mut checksum = 0_u64;
    for _ in 0..ITERATIONS {
        // `black_box` stops the optimiser from hoisting the call out of the
        // loop and turning the measurement into a no-op.
        checksum = checksum.wrapping_add(black_box(&order).total_cents());
    }
    let elapsed = start.elapsed();

    println!(
        "order-total: {LINES} lines x {ITERATIONS} iterations in {elapsed:?} \
         ({:?}/iteration, checksum {checksum})",
        elapsed / ITERATIONS
    );
}

fn build_order() -> Order {
    let mut order = Order::new(OrderId(1), CustomerId(7));
    for n in 0..LINES {
        let line = OrderLine::new(format!("sku-{n}"), n + 1, u64::from(n) * 100)
            .unwrap_or_else(|error| unreachable!("quantity is never zero: {error}"));
        order
            .push_line(line)
            .unwrap_or_else(|error| unreachable!("order is still a draft: {error}"));
    }
    order
}
