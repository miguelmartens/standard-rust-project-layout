//! Turning the process environment into an [`app_core::Config`].
//!
//! # Why the loading is here and the shape is in `app-core`
//!
//! `app_core::Config` derives `Deserialize` and stops. A library that reads
//! `std::env` itself has an input its caller cannot see, cannot override and
//! cannot vary between two tests in the same process. So the library owns the
//! shape, and the binary -- the one place that already knows it is a process
//! with an environment -- owns the loading.
//!
//! That split is the reason this module is four dozen lines and `app-core` has
//! no idea environment variables exist.
//!
//! # Why the reading is passed in as a closure
//!
//! Reading a variable is safe; *setting* one is not. `std::env::set_var` is
//! `unsafe` in edition 2024, because it mutates state another thread may be
//! reading, and `unsafe_code` is forbidden across this workspace anyway. A unit
//! test therefore cannot arrange an environment in-process at all.
//!
//! Taking the lookup as an argument sidesteps the problem rather than fighting
//! it: the parsing and the error messages, which are all of the behaviour worth
//! testing, become a pure function of a `&str -> Option<OsString>`. `from_env`
//! is then the single line that touches the real environment, and
//! `tests/cli-args.rs` covers it by setting variables on a *child* process,
//! which is safe and which is what a shell does anyway.
//!
//! # Why `OsString` and not `String`
//!
//! `std::env::var` collapses "unset" and "set to something that is not UTF-8"
//! into one error type, and the `.ok()` everyone reaches for on top of it then
//! treats a mistyped value as absent and quietly applies the default. `var_os`
//! keeps the two apart. Same reasoning as `deny_unknown_fields` on `Config`:
//! configuration that is wrong should say so rather than do something else.

use std::ffi::OsString;

use anyhow::{Context, Result, anyhow};
use app_core::Config;

/// ISO 4217 currency code. See [`Config::currency`].
const CURRENCY: &str = "APP_CURRENCY";

/// Cap on the number of lines in an order. See [`Config::max_order_lines`].
const MAX_ORDER_LINES: &str = "APP_MAX_ORDER_LINES";

/// Reads the configuration from the process environment.
///
/// Every variable is optional and an unset one keeps its [`Config::default`]
/// value, which is why `.env.example` documents the defaults rather than the
/// file being required to exist.
///
/// # Errors
///
/// If a variable is set to a value that is not valid UTF-8, or that does not
/// parse as the type of the field it feeds. Both messages name the variable:
/// `invalid digit found in string` on its own is not something an operator can
/// act on.
pub(crate) fn from_env() -> Result<Config> {
    // Not `from_lookup(std::env::var_os)`: that names one instantiation of a
    // generic function, and the argument here is higher-ranked over the key's
    // lifetime. The closure is what makes it so.
    from_lookup(|key: &str| std::env::var_os(key))
}

fn from_lookup(get: impl Fn(&str) -> Option<OsString>) -> Result<Config> {
    let mut config = Config::default();

    if let Some(raw) = get(CURRENCY) {
        config.currency = utf8(CURRENCY, raw)?;
    }

    if let Some(raw) = get(MAX_ORDER_LINES) {
        config.max_order_lines = utf8(MAX_ORDER_LINES, raw)?
            .parse()
            .with_context(|| format!("{MAX_ORDER_LINES} must be a whole number"))?;
    }

    Ok(config)
}

/// The offending value is deliberately absent from the message: environment
/// variables hold credentials, and error messages end up in logs.
fn utf8(key: &str, raw: OsString) -> Result<String> {
    raw.into_string()
        .map_err(|_| anyhow!("{key} is set to a value that is not valid UTF-8"))
}

#[cfg(test)]
mod tests {
    // Test code: a panic *is* the failure report.
    #![allow(clippy::unwrap_used)]

    use super::*;

    /// Stands in for the environment. Note that no test here mutates anything
    /// global, so they stay correct when run in parallel.
    fn load(vars: &[(&str, &str)]) -> Result<Config> {
        from_lookup(|key| {
            vars.iter()
                .find(|(name, _)| *name == key)
                .map(|(_, value)| OsString::from(*value))
        })
    }

    #[test]
    fn an_empty_environment_is_the_default_configuration() {
        assert_eq!(load(&[]).unwrap(), Config::default());
    }

    #[test]
    fn a_set_variable_overrides_the_default() {
        let config = load(&[("APP_CURRENCY", "GBP"), ("APP_MAX_ORDER_LINES", "5")]).unwrap();

        assert_eq!(config.currency, "GBP");
        assert_eq!(config.max_order_lines, 5);
    }

    #[test]
    fn an_unparseable_value_is_rejected_by_name() {
        let error = load(&[("APP_MAX_ORDER_LINES", "lots")]).unwrap_err();

        assert!(
            format!("{error}").contains("APP_MAX_ORDER_LINES"),
            "error was: {error}"
        );
    }
}
