//! Obscura Core Library
//!
//! Provides fundamental blockchain data structures and helpers.

use blake2::{Blake2b512, Digest};
use serde::{Deserialize, Serialize};

pub mod ledger;
pub mod pow;
mod block_ext;
use std::time::{SystemTime, UNIX_EPOCH};

pub type Hash = [u8; 32];

/// Returns the UNIX timestamp in seconds.
fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A reference to a previous unspent transaction output (UTXO) being spent.
///
/// Fields
/// -------
/// * `prev_tx` - 32-byte hash identifying the transaction that created the output.
/// * `output_index` - Position of the output inside `prev_tx`’s `outputs` vector.
/// * `pubkey` - Ed25519 public key (raw bytes) that authorises spending.
/// * `signature` - Ed25519 signature over the deterministic transaction message (see
///   [`ledger::Ledger::tx_message`]).
///
/// The signature must validate against `pubkey` and authorises the spend if the
/// referenced UTXO’s `pubkey_hash` matches `hash160(pubkey)` once address
/// encoding is implemented.
pub struct TxInput {
    pub prev_tx: Hash,
    pub output_index: u32,
    pub pubkey: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A newly created spendable output produced by a transaction.
///
/// Fields
/// -------
/// * `value` – Amount in “Obsc” (smallest currency unit, currently 1 == 1 Obsc)
///   carried by this output.
/// * `pubkey_hash` – Hash of the recipient’s public key. When they later spend
///   the output they will reveal the matching public key and a valid
///   signature.
pub struct TxOutput {
    pub value: u64,
    pub pubkey_hash: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a transfer of value.
///
/// A transaction destroys the UTXOs referenced by all [`TxInput`]s and creates
/// a new set of [`TxOutput`]s. It is valid if:
/// 1. Each input references an existing unspent output in the ledger.
/// 2. All signatures verify against the corresponding public keys.
/// 3. The sum of output `value`s does not exceed the sum of input `value`s
///    (remainder is treated as a transaction fee).
///
/// Coinbase (block-reward) transactions are special: they have an empty `inputs`
/// vector and may mint new coins up to the consensus-defined reward schedule.
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub metadata: Option<Vec<u8>>, // Optional extra data
}

impl Transaction {
    pub fn hash(&self) -> Hash {
        let encoded = bincode::serialize(self).expect("tx serialize");
        let digest = Blake2b512::digest(&encoded);
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&digest[..32]);
        hash
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Metadata identifying a block.
///
/// The header is the portion hashed during proof-of-work.  Changing *any* field
/// results in a completely different hash.
///
/// Fields
/// -------
/// * `index` – Height of the block (genesis == 1).
/// * `timestamp` – Seconds since Unix epoch.
/// * `prev_hash` – Hash of the previous block’s header (all zeros for genesis).
/// * `merkle_root` – Root hash of the merkle tree built from transaction
///   hashes.  Currently implemented as a simple concatenation hash for
///   scaffolding.
/// * `nonce` – Incremented during mining until the header hash satisfies the
///   target difficulty.
/// * `difficulty` – Target leading-zero bit count the hash must satisfy.
pub struct BlockHeader {
    pub index: u64,
    pub timestamp: u64,
    pub prev_hash: Hash,
    pub merkle_root: Hash,
    pub nonce: u64,
    pub difficulty: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A container for an ordered set of transactions plus a header linking it
/// into the blockchain.
///
/// The first transaction *must* be the coinbase rewarding the miner. All
/// subsequent transactions are validated against the ledger’s current UTXO set
/// before being applied in order.
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
        /// Constructs a new block with a freshly calculated merkle root and the
    /// current wall-clock timestamp.  The `nonce` starts at 0; callers should
    /// invoke [`crate::block_ext::Block::mine`] (via the `mine` extension
    /// method) to find a valid nonce.
    pub fn new(index: u64, prev_hash: Hash, transactions: Vec<Transaction>, difficulty: u32) -> Self {
        let merkle_root = Self::calc_merkle_root(&transactions);
        Self {
            header: BlockHeader {
                index,
                timestamp: now_ts(),
                prev_hash,
                merkle_root,
                nonce: 0,
                difficulty,
            },
            transactions,
        }
    }

        /// Computes the Merkle root of `txs`.
    ///
    /// NOTE: This is currently a **placeholder** implementation that simply
    /// hashes the concatenation of transaction hashes.  For production use we
    /// will replace it with a proper binary Merkle tree with duplicate handling
    /// (Bitcoin-style) which enables efficient SPV proofs.
    pub fn calc_merkle_root(txs: &[Transaction]) -> Hash {
        let mut hasher = Blake2b512::new();
        for tx in txs {
            hasher.update(tx.hash());
        }
        let result = hasher.finalize();
        let mut root = [0u8; 32];
        root.copy_from_slice(&result[..32]);
        root
    }

        /// Returns the Blake2b-256 hash of the block header.
    ///
    /// This hash functions as both the block identifier and the proof-of-work
    /// input.
    pub fn hash(&self) -> Hash {
        let encoded = bincode::serialize(&self.header).expect("header serialize");
        let digest = Blake2b512::digest(&encoded);
        let mut h = [0u8; 32];
        h.copy_from_slice(&digest[..32]);
        h
    }
}
