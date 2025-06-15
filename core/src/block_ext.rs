//! Extension methods for [`Block`] implemented outside `lib.rs` to keep the
//! core data-structure definition terse.
//!
//! The extensions cover common consensus-layer helpers that *require* the full
//! block context (transactions + header):
//!
//! * [`Block::is_valid`] – lightweight validation against PoW target, Merkle
//!   root and chain linkage.
//! * [`Block::mine`] – naïve single-threaded mining loop suitable for testing.
//!
//! Production code will replace `mine` with an async, multi-threaded miner and
//! `is_valid` will be expanded to enforce timestamp drift, difficulty limits
//! and consensus rules.

use crate::{pow, Hash, Block};

impl Block {
    /// Returns `true` if the block header hash meets difficulty and structural
    /// invariants.
    ///
    /// This check is *contextual* – it requires `expected_prev`, typically the
    /// current chain tip hash, to confirm proper linkage.
    pub fn is_valid(&self, expected_prev: &Hash) -> bool {
        self.header.prev_hash == *expected_prev
            && Self::calc_merkle_root(&self.transactions) == self.header.merkle_root
            && pow::hash_meets_difficulty(&self.hash(), self.header.difficulty)
    }

    /// Performs a naïve brute-force mining loop.
    ///
    /// Useful in unit tests where deterministic runtime is not critical.  The
    /// function consumes `self` and returns the mined block to avoid accidental
    /// reuse of a partially-modified instance.
    pub fn mine(mut self) -> Self {
        while !pow::hash_meets_difficulty(&self.hash(), self.header.difficulty) {
            self.header.nonce = self.header.nonce.wrapping_add(1);
        }
        self
    }
}
