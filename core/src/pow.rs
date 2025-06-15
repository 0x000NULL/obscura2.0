//! Proof-of-Work helpers.
//!
//! Currently the consensus algorithm is a simplified *leading-zero* target: a
//! hash is valid if it begins with `difficulty` zero bits.  In production we
//! will switch to a proper *target value* representation compatible with
//! Bitcoin so difficulty can be adjusted by changing the target, not the bit
//! count.
//!
//! All functions are pure and stateless so they can be used from any thread.

use crate::Hash;

/// Returns `true` if `hash` meets the difficulty target.
///
/// Difficulty is expressed as a **count of leading zero bits** (0-256).  For
/// example:
///
/// * `difficulty == 0` → always valid.
/// * `difficulty == 8` → hash must start with one `0x00` byte.
/// * `difficulty == 12` → first byte `0x00`, second byte`s` high 4 bits zero.
pub fn hash_meets_difficulty(hash: &Hash, difficulty: u32) -> bool {
pub fn hash_meets_difficulty(hash: &Hash, difficulty: u32) -> bool {
    if difficulty == 0 {
        return true;
    }
    let zero_bytes = (difficulty / 8) as usize;
    let zero_bits = (difficulty % 8) as u8;

    // full bytes must be zero
    if hash.iter().take(zero_bytes).any(|&b| b != 0) {
        return false;
    }

    if zero_bits == 0 {
        return true;
    }
    let next_byte = hash[zero_bytes];
    next_byte.leading_zeros() as u8 >= zero_bits
}
