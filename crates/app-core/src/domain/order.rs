//! Orders and their lifecycle.
//!
//! # Why the unit tests at the bottom of this file
//!
//! "You'll put unit tests in the `src` directory in each file with the code
//! that they're testing. The convention is to create a module named `tests` in
//! each file to contain the test functions and to annotate the module with
//! `cfg(test)`."
//! — <https://doc.rust-lang.org/book/ch11-03-test-organization.html>
//!
//! Because `mod tests` is a child module, `use super::*` gives it access to
//! private fields and private functions. Compare with
//! `tests/order-lifecycle.rs`, which is a separate crate and can only reach
//! what `lib.rs` re-exports. The two are testing different things on purpose;
//! neither replaces the other.
//!
//! There is no mirrored `test/` tree anywhere in this repo, and there should
//! not be one in yours.

use std::fmt;

use crate::config::Config;
use crate::domain::customer::CustomerId;
use crate::error::{Error, Result};

/// An order's identity. See [`CustomerId`] for why this is a newtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderId(pub u64);

/// Where an order is in its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum OrderStatus {
    /// Being assembled. The only state in which lines may be added.
    Draft,
    /// Submitted by the customer.
    Placed,
    /// Handed to the carrier.
    Shipped,
    /// Abandoned before shipping.
    Cancelled,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implemented by hand rather than derived, because this string is shown
        // to humans (it lands in `Error::InvalidTransition`) and so is part of
        // the API. `Debug` output is not something to promise stability for.
        let name = match self {
            Self::Draft => "draft",
            Self::Placed => "placed",
            Self::Shipped => "shipped",
            Self::Cancelled => "cancelled",
        };
        f.write_str(name)
    }
}

/// One line on an order: a SKU, a quantity, and a unit price.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderLine {
    sku: String,
    quantity: u32,
    unit_price_cents: u64,
}

impl OrderLine {
    /// Creates a line, rejecting a zero quantity.
    ///
    /// Money is stored in minor units (cents) as an integer. Floating point
    /// money is a bug waiting to be filed.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidQuantity`] if `quantity` is zero.
    pub fn new(sku: impl Into<String>, quantity: u32, unit_price_cents: u64) -> Result<Self> {
        let sku = sku.into();
        if quantity == 0 {
            return Err(Error::InvalidQuantity { sku });
        }
        Ok(Self {
            sku,
            quantity,
            unit_price_cents,
        })
    }

    /// The stock keeping unit this line refers to.
    #[must_use]
    pub fn sku(&self) -> &str {
        &self.sku
    }

    /// How many units were ordered. Never zero — see [`OrderLine::new`].
    #[must_use]
    pub const fn quantity(&self) -> u32 {
        self.quantity
    }

    /// The price of a single unit, in cents.
    #[must_use]
    pub const fn unit_price_cents(&self) -> u64 {
        self.unit_price_cents
    }

    /// Quantity multiplied by unit price, in cents.
    ///
    /// Saturates rather than wrapping. An order that overflows a `u64` of cents
    /// is a data-entry error, and reporting an absurdly large number is easier
    /// to notice than a small one that wrapped around.
    #[must_use]
    pub fn subtotal_cents(&self) -> u64 {
        u64::from(self.quantity).saturating_mul(self.unit_price_cents)
    }
}

/// A customer's order, and the rules about how it may change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    id: OrderId,
    customer: CustomerId,
    // Private. The invariant "lines are only added while `status` is `Draft`"
    // is enforced by `push_line`, and can only stay true because no caller can
    // reach `lines` directly.
    lines: Vec<OrderLine>,
    status: OrderStatus,
}

impl Order {
    /// Starts a new, empty draft order.
    #[must_use]
    pub const fn new(id: OrderId, customer: CustomerId) -> Self {
        Self {
            id,
            customer,
            lines: Vec::new(),
            status: OrderStatus::Draft,
        }
    }

    /// The order's identity.
    #[must_use]
    pub const fn id(&self) -> OrderId {
        self.id
    }

    /// Who the order belongs to.
    #[must_use]
    pub const fn customer(&self) -> CustomerId {
        self.customer
    }

    /// Where the order is in its lifecycle.
    #[must_use]
    pub const fn status(&self) -> OrderStatus {
        self.status
    }

    /// The lines on the order, in the order they were added.
    #[must_use]
    pub fn lines(&self) -> &[OrderLine] {
        &self.lines
    }

    /// Appends a line to a draft order.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidTransition`] if the order has already been
    /// placed, shipped, or cancelled.
    pub fn push_line(&mut self, line: OrderLine) -> Result<()> {
        if self.status != OrderStatus::Draft {
            return Err(Error::InvalidTransition {
                from: self.status,
                to: OrderStatus::Draft,
            });
        }
        self.lines.push(line);
        Ok(())
    }

    /// The sum of every line's subtotal, in cents.
    ///
    /// # Examples
    ///
    /// ```
    /// use app_core::{CustomerId, Order, OrderId, OrderLine};
    ///
    /// let mut order = Order::new(OrderId(1), CustomerId(1));
    /// order.push_line(OrderLine::new("widget", 2, 500)?)?;
    /// order.push_line(OrderLine::new("gadget", 1, 250)?)?;
    ///
    /// assert_eq!(order.total_cents(), 1_250);
    /// # Ok::<(), app_core::Error>(())
    /// ```
    #[must_use]
    pub fn total_cents(&self) -> u64 {
        self.lines
            .iter()
            .map(OrderLine::subtotal_cents)
            .fold(0, u64::saturating_add)
    }

    /// Submits the order.
    ///
    /// This is the one method that needs the [`Config`], which is why the
    /// configuration is passed in rather than read from a global. A global
    /// would make every test in this file depend on process-wide state.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidTransition`] if the order is not a draft.
    /// - [`Error::EmptyOrder`] if it has no lines.
    /// - [`Error::TooManyLines`] if it exceeds [`Config::max_order_lines`].
    pub fn place(&mut self, config: &Config) -> Result<()> {
        if self.status != OrderStatus::Draft {
            return Err(Error::InvalidTransition {
                from: self.status,
                to: OrderStatus::Placed,
            });
        }
        if self.lines.is_empty() {
            return Err(Error::EmptyOrder);
        }
        if self.lines.len() > config.max_order_lines {
            return Err(Error::TooManyLines {
                actual: self.lines.len(),
                max: config.max_order_lines,
            });
        }
        self.status = OrderStatus::Placed;
        Ok(())
    }

    /// Marks a placed order as shipped.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidTransition`] unless the order is placed.
    pub fn ship(&mut self) -> Result<()> {
        if self.status != OrderStatus::Placed {
            return Err(Error::InvalidTransition {
                from: self.status,
                to: OrderStatus::Shipped,
            });
        }
        self.status = OrderStatus::Shipped;
        Ok(())
    }

    /// Cancels an order that has not shipped.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidTransition`] if the order has already shipped or
    /// was already cancelled.
    pub fn cancel(&mut self) -> Result<()> {
        if matches!(self.status, OrderStatus::Shipped | OrderStatus::Cancelled) {
            return Err(Error::InvalidTransition {
                from: self.status,
                to: OrderStatus::Cancelled,
            });
        }
        self.status = OrderStatus::Cancelled;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // The workspace denies `unwrap`/`expect` because they are a liability in
    // shipped code. In a test, a panic *is* the failure report, so the lints
    // are switched off for this module and nowhere else.
    #![allow(clippy::unwrap_used)]

    use super::*;

    fn draft_with_one_line() -> Order {
        let mut order = Order::new(OrderId(1), CustomerId(7));
        order
            .push_line(OrderLine::new("widget", 2, 500).unwrap())
            .unwrap();
        order
    }

    // ---- The point of these tests: they reach private state. ----------------
    //
    // `order.status` and `order.lines` are private fields. Only code inside the
    // module can name them, and `mod tests` is inside the module. An
    // integration test in `tests/` cannot write any of the assertions below.

    #[test]
    fn a_new_order_is_an_empty_draft() {
        let order = Order::new(OrderId(1), CustomerId(7));

        assert_eq!(order.status, OrderStatus::Draft);
        assert!(order.lines.is_empty());
    }

    #[test]
    fn placing_an_order_flips_the_private_status_field() {
        let mut order = draft_with_one_line();

        order.place(&Config::default()).unwrap();

        assert_eq!(order.status, OrderStatus::Placed);
    }

    #[test]
    fn lines_are_appended_in_order() {
        let mut order = Order::new(OrderId(1), CustomerId(7));
        order
            .push_line(OrderLine::new("a", 1, 100).unwrap())
            .unwrap();
        order
            .push_line(OrderLine::new("b", 1, 200).unwrap())
            .unwrap();

        let skus: Vec<&str> = order.lines.iter().map(OrderLine::sku).collect();
        assert_eq!(skus, ["a", "b"]);
    }

    // ---- Ordinary behaviour tests, which happen to be easiest to write here --

    #[test]
    fn a_zero_quantity_line_is_rejected() {
        let error = OrderLine::new("widget", 0, 500).unwrap_err();

        assert!(matches!(error, Error::InvalidQuantity { ref sku } if sku == "widget"));
    }

    #[test]
    fn an_empty_order_cannot_be_placed() {
        let mut order = Order::new(OrderId(1), CustomerId(7));

        assert!(matches!(
            order.place(&Config::default()),
            Err(Error::EmptyOrder)
        ));
    }

    #[test]
    fn lines_cannot_be_added_after_placing() {
        let mut order = draft_with_one_line();
        order.place(&Config::default()).unwrap();

        let error = order
            .push_line(OrderLine::new("late", 1, 100).unwrap())
            .unwrap_err();

        assert!(matches!(
            error,
            Error::InvalidTransition {
                from: OrderStatus::Placed,
                to: OrderStatus::Draft
            }
        ));
    }

    #[test]
    fn the_line_limit_is_taken_from_the_config() {
        let config = Config {
            max_order_lines: 1,
            ..Config::default()
        };
        let mut order = draft_with_one_line();
        order
            .push_line(OrderLine::new("second", 1, 100).unwrap())
            .unwrap();

        assert!(matches!(
            order.place(&config),
            Err(Error::TooManyLines { actual: 2, max: 1 })
        ));
    }

    #[test]
    fn a_shipped_order_cannot_be_cancelled() {
        let mut order = draft_with_one_line();
        order.place(&Config::default()).unwrap();
        order.ship().unwrap();

        assert!(order.cancel().is_err());
    }

    #[test]
    fn subtotals_saturate_instead_of_overflowing() {
        let line = OrderLine::new("bulk", u32::MAX, u64::MAX).unwrap();

        assert_eq!(line.subtotal_cents(), u64::MAX);
    }
}
