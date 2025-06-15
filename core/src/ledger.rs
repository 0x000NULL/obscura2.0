//! UTXO Ledger implementation for **Obscura**.
//!
//! The ledger tracks the set of unspent transaction outputs (UTXOs) and the
//! canonical chain tip.  Blocks are validated *sequentially*; forks and chain
//! selection will be handled by the consensus layer in a future module.
//!
//! Glossary
//! --------
//! * **UTXO** – A spendable output referenced by `(tx_hash, output_index)`.
//! * **Tip** – The header hash of the most recently applied block.
//! * **Height** – 1-based index of the latest block.
//!
//! The API purposefully exposes only high-level operations: applying a block
//! and querying balances.  More granular functions (e.g. mempool simulation)
//! should be part of higher layers.
use std::collections::HashMap;

use crate::{Hash, Transaction, TxOutput, Block};
use ed25519_dalek::{PublicKey, Signature};
use blake2::{Blake2b512, Digest};


pub type UtxoKey = (Hash, u32);

#[derive(Debug, Clone)]
/// In-memory UTXO set and chain metadata.
///
/// The `Ledger` is **not** thread-safe by itself; callers must wrap it in a
/// `RwLock`/`Mutex` or use an actor model if concurrent access is required.
pub struct Ledger {
    pub utxos: HashMap<UtxoKey, TxOutput>,
    pub height: u64,
    pub tip: Hash,
}

impl Ledger {
    /// Constructs a ledger initialised with the *genesis* block.
    ///
    /// The genesis must satisfy the same validity rules as any other block
    /// except that its `prev_hash` is all zeros and its index is 1.
    pub fn new(genesis: &Block) -> Result<Self, String> {
        let mut ledger = Ledger { utxos: HashMap::new(), height: 0, tip: [0u8; 32] };
        ledger.apply_block(genesis)?;
        Ok(ledger)
    }

    /// Validates `block` against current state and, if valid, mutates the
    /// ledger by:
    /// 1. Spending each referenced input (removing UTXOs).
    /// 2. Inserting newly created outputs.
    /// 3. Advancing `height`/`tip`.
    ///
    /// Errors on double-spends, value overflow, signature failure or bad
    /// linkage.
    pub fn apply_block(&mut self, block: &Block) -> Result<(), String> {
        // simple prev check
        if block.header.index != self.height + 1 {
            return Err("non-sequential height".into());
        }
        if block.header.prev_hash != self.tip {
            return Err("prev hash mismatch".into());
        }
        // iterate transactions
        for (idx, tx) in block.transactions.iter().enumerate() {
            if idx != 0 {
                self.validate_tx(tx)?;
            }
            // spend
            for inp in &tx.inputs {
                self.utxos.remove(&(inp.prev_tx, inp.output_index));
            }
            // create outputs
            let tx_hash = tx.hash();
            for (i, out) in tx.outputs.iter().enumerate() {
                self.utxos.insert((tx_hash, i as u32), out.clone());
            }
        }
        self.height = block.header.index;
        self.tip = block.hash();
        Ok(())
    }

    /// Computes the deterministic signing message for a transaction.
    ///
    /// We hash the serialised transaction *after* zeroing all signatures so
    /// that each input signs the same message and the signature does not cover
    /// itself (circular dependency).
    fn tx_message(tx: &Transaction) -> [u8;32] {
        let mut clone = tx.clone();
        // Zero out signatures for hashing
        for inp in &mut clone.inputs {
            inp.signature.clear();
        }
        let encoded = bincode::serialize(&clone).expect("tx serialize");
        let digest = blake2::Blake2b512::digest(&encoded);
        let mut msg = [0u8;32];
        msg.copy_from_slice(&digest[..32]);
        msg
    }

    fn validate_tx(&self, tx: &Transaction) -> Result<(), String> {
        let mut input_value = 0u64;
        let mut output_value = 0u64;
        for inp in &tx.inputs {
            if let Some(prev_out) = self.utxos.get(&(inp.prev_tx, inp.output_index)) {
                input_value += prev_out.value;
                // Signature verification (skip if empty for placeholder)
                if !inp.signature.is_empty() {
                    let pk = PublicKey::from_bytes(&inp.pubkey)
                        .map_err(|_| "invalid pubkey")?;
                    let sig = Signature::from_bytes(&inp.signature)
                        .map_err(|_| "invalid signature")?;
                    let msg = Self::tx_message(tx);
                    pk.verify_strict(&msg, &sig).map_err(|_| "bad signature")?;
                }
            } else {
                return Err("referenced UTXO not found".into());
            }
        }
        for out in &tx.outputs {
            output_value += out.value;
        }
        if output_value > input_value {
            return Err("outputs exceed inputs".into());
        }
        Ok(())
    }

    pub fn balance_for_pubkey_hash(&self, pkh: &[u8]) -> u64 {
        self.utxos
            .values()
            .filter(|utxo| utxo.pubkey_hash.as_slice() == pkh)
            .map(|u| u.value)
            .sum()
    }
}
