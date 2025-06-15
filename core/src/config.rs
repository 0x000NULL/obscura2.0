//! Consensus and runtime configuration for **Obscura** core.
//!
//! The [`Config`] struct centralises tunable parameters such as difficulty
//! targets and coinbase reward schedule. It is constructed via the
//! [`ConfigBuilder`] using the fluent builder pattern, enabling callers to
//! customise only the fields they care about while keeping sensible defaults.
//!
//! All fields are `pub` so read-only access is ergonomic, however mutation
//! should occur through the builder to preserve validation invariants.
//!
//! ```
//! use obscura_core::config::Config;
//!
//! // default main-net configuration
//! let cfg = Config::default();
//! assert_eq!(cfg.difficulty, 8);
//! ```

use serde::{Deserialize, Serialize};

/// Runtime configuration shared across the crate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    /// PoW leading-zero difficulty in bits.
    pub difficulty: u32,

    /// Block subsidy in „Obsc“ paid to the miner.
    pub block_reward: u64,

    /// Human-readable name identifying the network (e.g. "main", "test").
    pub network: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            difficulty: 8,
            block_reward: 50,
            network: "main".into(),
        }
    }
}

/// Fluent builder for [`Config`].
pub struct ConfigBuilder {
    inner: Config,
}

impl ConfigBuilder {
    /// Starts a new builder pre-populated with [`Config::default`].
    pub fn new() -> Self {
        Self { inner: Config::default() }
    }

    pub fn difficulty(mut self, diff: u32) -> Self {
        self.inner.difficulty = diff;
        self
    }

    pub fn block_reward(mut self, reward: u64) -> Self {
        self.inner.block_reward = reward;
        self
    }

    pub fn network<S: Into<String>>(mut self, name: S) -> Self {
        self.inner.network = name.into();
        self
    }

    /// Consumes the builder returning an immutable configuration value.
    pub fn finish(self) -> Config {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_overrides_fields() {
        let cfg = ConfigBuilder::new()
            .difficulty(16)
            .block_reward(25)
            .network("test")
            .finish();
        assert_eq!(cfg.difficulty, 16);
        assert_eq!(cfg.block_reward, 25);
        assert_eq!(cfg.network, "test");
    }
}
