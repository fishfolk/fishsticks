//! Common types for error handling.

use std::result;

/// Common error type
pub type Error = String;

/// Common result type
pub type Result<T> = result::Result<T, Error>;
