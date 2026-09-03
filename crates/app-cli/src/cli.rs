//! Argument parsing and wiring. No business rules.
//!
//! # Why `anyhow` here and `thiserror` in `app-core`
//!
//! Nothing downstream will ever `match` on an error out of this crate — the
//! only consumer is a human reading stderr. That makes the ability to attach
//! context far more valuable than the ability to enumerate variants, which is
//! precisely the trade `anyhow` makes.
//!
//! The rule of thumb: `thiserror` for libraries, `anyhow` for binaries. It
//! follows from who the caller is, not from taste.
//!
//! # Why everything here is `pub(crate)`
//!
//! `unreachable_pub` is on for the workspace. In a binary crate nothing is
//! reachable from outside, so a bare `pub` is always a lie and the lint says
//! so. `pub(crate)` states the actual intent.

use anyhow::{Context, Result, bail};
use app_core::{Config, CustomerId, Order, OrderId, OrderLine};
use clap::{Parser, Subcommand};

/// Uppercased at compile time by `app-macros`.
///
/// This exists to show a proc-macro crate being consumed by a sibling through
/// `[workspace.dependencies]`. In a real CLI you would just write the string.
const BANNER: &str = app_macros::shout!("app - the example order tool");

/// The top-level command line.
#[derive(Debug, Parser)]
#[command(name = "app", version, about = BANNER, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Total up an order given on the command line.
    Total {
        /// One order line, as `SKU:QUANTITY:UNIT_PRICE_CENTS`. Repeatable.
        #[arg(long = "line", value_name = "SKU:QTY:CENTS", required = true)]
        lines: Vec<String>,
    },
    /// Print the effective configuration.
    Config,
}

impl Cli {
    /// Runs the parsed command.
    ///
    /// # Errors
    ///
    /// Returns any failure from argument interpretation or from `app-core`,
    /// with enough context attached to be actionable on stderr.
    pub(crate) fn run(self) -> Result<()> {
        let config = Config::default();

        match self.command {
            Command::Total { lines } => total(&lines, &config),
            Command::Config => {
                println!("{config:#?}");
                Ok(())
            }
        }
    }
}

/// Builds an order out of `--line` arguments and prints its total.
///
/// Note how little happens here: parse strings into domain types, call one
/// domain method, format the result. Every rule being exercised — that a
/// quantity cannot be zero, that an empty order cannot be placed, that the line
/// count is capped — lives in `app-core` and is tested there without a process.
fn total(raw_lines: &[String], config: &Config) -> Result<()> {
    let mut order = Order::new(OrderId(1), CustomerId(1));

    for raw in raw_lines {
        let line = parse_line(raw).with_context(|| format!("invalid --line value `{raw}`"))?;
        order.push_line(line)?;
    }

    order.place(config).context("the order was rejected")?;

    println!(
        "{} lines, total {} {}.{:02}",
        order.lines().len(),
        config.currency,
        order.total_cents() / 100,
        order.total_cents() % 100
    );
    Ok(())
}

/// Parses `SKU:QUANTITY:UNIT_PRICE_CENTS`.
///
/// A hand-rolled parser rather than a clap `value_parser` so the error-context
/// chain stays visible in this example. Anything more structured than this
/// belongs in a `FromStr` impl next to the type it produces.
fn parse_line(raw: &str) -> Result<OrderLine> {
    let mut parts = raw.split(':');

    let (Some(sku), Some(quantity), Some(price), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        bail!("expected exactly three colon-separated fields");
    };

    let quantity = quantity
        .parse()
        .context("quantity must be a non-negative integer")?;
    let price = price
        .parse()
        .context("unit price must be a non-negative integer number of cents")?;

    Ok(OrderLine::new(sku, quantity, price)?)
}
