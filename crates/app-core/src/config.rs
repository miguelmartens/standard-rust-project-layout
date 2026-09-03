//! Configuration *shape*, not configuration *loading*.
//!
//! # Why loading is not here
//!
//! This module derives [`serde::Deserialize`] and stops. It does not depend on
//! `toml`, `serde_json`, `figment`, or `config`, and it never touches the
//! filesystem.
//!
//! Choosing a file format is an application decision. A library that picks one
//! forces it on every consumer and drags the parser into every dependency tree
//! that touches it. Deriving `Deserialize` costs nothing and lets the binary
//! decide: `app-cli` could read TOML, a server could read environment
//! variables, a test can build the struct literally.
//!
//! This is the same reasoning that keeps `serde` itself format-agnostic.

use serde::{Deserialize, Serialize};

/// Runtime limits and presentation settings for the domain.
///
/// # Why `deny_unknown_fields`
///
/// Without it, serde silently ignores keys it does not recognise. A typo like
/// `max_order_line` (singular) would parse fine and quietly apply the default,
/// and the operator would debug the wrong thing for an hour.
///
/// The trade-off is that removing a field becomes a breaking change for anyone
/// with it still in their config file. For an application's own configuration
/// that is a good trade. For a wire format you must stay compatible with, it is
/// the wrong one — there, silently ignoring unknown fields is the whole point.
///
/// # Examples
///
/// ```
/// use app_core::Config;
///
/// let config = Config::default();
/// assert_eq!(config.currency, "EUR");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// ISO 4217 currency code used when rendering money.
    #[serde(default = "default_currency")]
    pub currency: String,

    /// Refuse to place an order with more lines than this.
    #[serde(default = "default_max_order_lines")]
    pub max_order_lines: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            currency: default_currency(),
            max_order_lines: default_max_order_lines(),
        }
    }
}

// Private helpers. `#[serde(default = "...")]` takes a path to a function, so
// these exist purely to keep `Default` and the serde defaults in agreement:
// one source of truth, used twice.
fn default_currency() -> String {
    "EUR".to_owned()
}

const fn default_max_order_lines() -> usize {
    100
}
