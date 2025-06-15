//! Error types for the **Obscura** core crate.
//!
//! All high-level operations return [`crate::Result`] which is a convenient
//! alias for `core::result::Result<T, Error>`.
//!
//! The enum is intentionally minimal and high-level.  Lower-level errors are
//! mapped into one of these variants before bubbling up to callers.
//!
//! # Examples
//!
//! ```
//! use obscura_core::{Error, Result};
//!
//! fn demo_fn(fail: bool) -> Result<()> {
//!     if fail {
//!         Err(Error::DifficultyFail)
//!     } else {
//!         Ok(())
//!     }
//! }
//! ```

use thiserror::Error;

/// Core crate error type.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum Error {
    /// Signature could not be verified against the provided public key.
    #[error("signature verification failed")]
    BadSignature,

    /// Referenced UTXO is absent from the current ledger state.
    #[error("referenced UTXO not found")]
    MissingUtxo,

    /// Attempted to spend the same output more than once in a single block.
    #[error("double spend attempted")]
    DoubleSpend,

    /// Sum of transaction outputs exceeds sum of inputs.
    #[error("value outputs exceed inputs")]
    ValueOverflow,

    /// `prev_hash` field does not match tip hash.
    #[error("block previous hash mismatch")]
    PrevHashMismatch,

    /// Block height is not exactly one greater than current height.
    #[error("block height non-sequential")]
    NonSequentialHeight,

    /// Block header hash does not satisfy the difficulty target.
    #[error("difficulty target not met")]
    DifficultyFail,

    /// Placeholder for errors originating from external crates.
    #[error("{0}")]
    Other(&'static str),
}

/// Convenient result alias used throughout the crate.
pub type Result<T> = core::result::Result<T, Error>;
