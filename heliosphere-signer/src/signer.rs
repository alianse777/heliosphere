//! Signer utils

use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Debug;
use heliosphere_core::transaction::Transaction;
use heliosphere_core::Address;
use k256::{
    ecdsa::{recoverable::Signature as RecoverableSignature, VerifyingKey},
    elliptic_curve::sec1::ToEncodedPoint,
};
use sha3::{Digest, Keccak256};

const TRON_MESSAGE_PREFIX: &[u8] = b"\x19TRON Signed Message:\n";

/// Derive Tron address from VerifyingKey
pub fn derive_address(key: &VerifyingKey) -> Address {
    let verifying_key = key.to_encoded_point(false);
    let x = verifying_key.x().unwrap();
    let y = verifying_key.y().unwrap();
    let mut p_bytes = Vec::new();
    p_bytes.extend_from_slice(&[0x04]);
    p_bytes.extend_from_slice(x.as_slice());
    p_bytes.extend_from_slice(y.as_slice());
    if p_bytes.len() == 65 {
        p_bytes.remove(0);
    }
    let hash = Keccak256::digest(&p_bytes);
    let mut addr = [0x41; 21];
    addr[1..].copy_from_slice(&hash.as_slice()[hash.len() - 20..]);
    Address::new(addr).unwrap()
}

/// Compute keccak256([b"\x19TRON Signed Message:\n", message.len().to_string().as_bytes(), message])
pub fn hash_message(message: &[u8]) -> Keccak256 {
    let mut buf = Vec::new();
    buf.extend_from_slice(TRON_MESSAGE_PREFIX);
    buf.extend_from_slice(message.len().to_string().as_bytes());
    buf.extend_from_slice(message);
    let mut digest = Keccak256::new();
    digest.update(&buf);
    digest
}

/// Generic signer
pub trait Signer {
    /// Signer error
    type Error: Debug + Send + Sync; //no Error in core for now

    /// Get public key
    fn public_key(&self) -> VerifyingKey;

    /// Get Tron address
    fn address(&self) -> Address {
        derive_address(&self.public_key())
    }

    /// Sign hashed value
    fn sign_prehash(&self, prehash: &[u8]) -> Result<RecoverableSignature, Self::Error>;

    /// Sign transaction
    fn sign_transaction(&self, tx: &mut Transaction) -> Result<(), Self::Error> {
        let signature = self.sign_prehash(&tx.tx_id.0)?;
        tx.signature.push(hex::encode(signature));
        Ok(())
    }
}
