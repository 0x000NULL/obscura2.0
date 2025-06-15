use obscura_core::{pow, ledger::Ledger, Block, Transaction, TxInput, TxOutput, Hash};
use ed25519_dalek::{Keypair, Signer, SecretKey, PublicKey};
use blake2::{Blake2b512, Digest};

fn zeros_hash() -> Hash { [0u8; 32] }

#[test]
fn pow_zero_difficulty_passes() {
    let random_hash = [0xAAu8; 32];
    assert!(pow::hash_meets_difficulty(&random_hash, 0));
}

#[test]
fn mining_produces_valid_block() {
    // simple coinbase tx
    let coinbase = Transaction {
        inputs: vec![],
        outputs: vec![TxOutput { value: 50, pubkey_hash: vec![1, 2, 3] }],
        metadata: None,
    };
    let block = Block::new(1, zeros_hash(), vec![coinbase], 8).mine(); // diff 8 bits
    assert!(pow::hash_meets_difficulty(&block.hash(), 8));
}

#[test]
fn ledger_applies_block() {
    // deterministic keypair for tests
    let secret_bytes = [42u8; 32];
    let secret = SecretKey::from_bytes(&secret_bytes).unwrap();
    let public = PublicKey::from(&secret);
    let keypair = Keypair { secret, public };
    let pkh = keypair.public.as_bytes().to_vec();

    // create genesis block with a coinbase paying 50 to the owner
    let coinbase = Transaction {
        inputs: vec![],
        outputs: vec![TxOutput { value: 50, pubkey_hash: pkh.clone() }],
        metadata: None,
    };
    let genesis = Block::new(1, zeros_hash(), vec![coinbase.clone()], 0);
    let mut ledger = Ledger::new(&genesis).expect("create ledger");
    assert_eq!(ledger.height, 1);
    assert_eq!(ledger.balance_for_pubkey_hash(&pkh), 50);

    // build a spend transaction:
    //  - spends the 50 coinbase output
    //  - sends 30 to a new address [4,5,6]
    //  - sends 20 back to the owner as change
    let mut spend_tx = Transaction {
        inputs: vec![TxInput {
            prev_tx: coinbase.hash(),
            output_index: 0,
            pubkey: keypair.public.as_bytes().to_vec(),
            signature: vec![],
        }],
        outputs: vec![
            TxOutput { value: 30, pubkey_hash: vec![4,5,6] },
            TxOutput { value: 20, pubkey_hash: pkh.clone() },
        ],
        metadata: None,
    };

    // sign the transaction
    let msg = {
        let mut unsigned = spend_tx.clone();
        unsigned.inputs[0].signature.clear();
        let enc = bincode::serialize(&unsigned).unwrap();
        let digest = Blake2b512::digest(&enc);
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&digest[..32]);
        bytes
    };
    let sig = keypair.sign(&msg);
    spend_tx.inputs[0].signature = sig.to_bytes().to_vec();

    // create a block containing the spend transaction
    let block2 = Block::new(2, ledger.tip, vec![spend_tx.clone()], 0);
    ledger.apply_block(&block2).expect("apply block2");

    assert_eq!(ledger.height, 2);
    // owner now has only the 20 change
    assert_eq!(ledger.balance_for_pubkey_hash(&pkh), 20);
    // recipient has 30
    assert_eq!(ledger.balance_for_pubkey_hash(&[4,5,6]), 30);
}

