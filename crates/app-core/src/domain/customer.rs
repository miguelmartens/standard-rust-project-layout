//! Customers.
//!
//! Deliberately thin. It is here to show that a `domain/` directory holds one
//! file per concept, not one file per layer — there is no `models.rs`,
//! `services.rs`, `repositories.rs` split, because those names describe
//! plumbing rather than the problem.

use serde::{Deserialize, Serialize};

/// A customer's identity.
///
/// A newtype rather than a bare `u64`: it costs nothing at runtime and makes
/// `Order::new(customer_id, order_id)` a compile error instead of a support
/// ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CustomerId(pub u64);

/// Someone who can place an order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Customer {
    // Private fields, with accessors below. The struct can gain an invariant
    // later (a validated email, say) without that being a breaking change.
    id: CustomerId,
    name: String,
}

impl Customer {
    /// Creates a customer.
    #[must_use]
    pub fn new(id: CustomerId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }

    /// The customer's identity.
    #[must_use]
    pub const fn id(&self) -> CustomerId {
        self.id
    }

    /// The customer's display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}
