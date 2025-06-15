//! Obscura Wallet library placeholder.

use ed25519_dalek::{Keypair, PublicKey, SecretKey};

/// Placeholder deterministic keypair (DO NOT USE IN PRODUCTION).
pub fn generate_keypair() -> Keypair {
    // 32 zero bytes as secret — insecure placeholder.
    let secret = SecretKey::from_bytes(&[0u8; 32]).expect("secret");
    let public = PublicKey::from(&secret);
    Keypair { secret, public }
}
