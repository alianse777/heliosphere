//! Wallet utils

use k256::ecdsa::{
    signature::hazmat::PrehashSigner, RecoveryId, Signature, SigningKey, VerifyingKey,
};
use rand_core::{CryptoRng, RngCore};

use crate::error::SignerError;

/// Error type for signer trait
pub type KeypairSignError = k256::ecdsa::Error;

/// Wallet containing public & private keys
pub struct Keypair {
    verifying_key: VerifyingKey,
    signing_key: SigningKey,
}

impl Keypair {
    /// Create new keypair from signing key
    pub fn from_signing_key(signing_key: SigningKey) -> Self {
        Self {
            verifying_key: *signing_key.verifying_key(),
            signing_key,
        }
    }

    /// Generate new keypair
    pub fn generate<R>(rng: &mut R) -> Self
    where
        R: RngCore + CryptoRng,
    {
        let signing_key = SigningKey::random(rng);
        Self::from_signing_key(signing_key)
    }

    /// Init from hex private key
    pub fn from_hex_key(key: &str) -> Result<Self, SignerError> {
        let bytes = hex::decode(key).map_err(|_| SignerError::KeyDecodeError)?;
        let signing_key = SigningKey::from_slice(&bytes).map_err(|_| SignerError::InvalidKey)?;
        Ok(Self::from_signing_key(signing_key))
    }

    /// Get public (verifying) key
    pub fn public_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }

    /// Get private (signing) key
    pub fn private_key(&self) -> &SigningKey {
        &self.signing_key
    }
}

impl crate::signer::Signer for Keypair {
    type Error = KeypairSignError;

    fn public_key(&self) -> VerifyingKey {
        self.verifying_key
    }

    fn sign_prehash(&self, prehash: &[u8]) -> Result<(Signature, RecoveryId), Self::Error> {
        let s: Signature = self.signing_key.sign_prehash(prehash)?;
        let rec_id = RecoveryId::trial_recovery_from_prehash(&self.verifying_key, prehash, &s)?;
        Ok((s, rec_id))
    }
}

#[cfg(test)]
mod test {
    use alloc::format;

    use crate::signer::Signer;

    use super::*;

    #[test]
    fn test_address() {
        let keypair = Keypair::from_hex_key(
            "b224f69fe10604d71263971ecd9cdc9f2fc59d0e7ddb6c9df2aa8631423d8cb0",
        )
        .unwrap();
        let address = keypair.address();
        assert_eq!(address.as_base58(), "TJ4bdYW5G7EXrzWJY1e1nduY3ihEzU1G4R");
    }

    #[test]
    fn test_tx_sign() {
        let keypair = Keypair::from_hex_key(
            "b224f69fe10604d71263971ecd9cdc9f2fc59d0e7ddb6c9df2aa8631423d8cb0",
        )
        .unwrap();
        let txid = hex::decode("1471a47a19f8cc87933af763a8a9bb579b1fdaad2cb55fe7587a2e01a6cce6fe")
            .unwrap();
        let (s, rec_id) = keypair.sign_prehash(&txid).unwrap();
        assert_eq!(format!("{}{:02}", hex::encode(s.to_bytes()), rec_id.to_byte()), "e713bf98011b64960d423ec1b80518ef7708d202d7de37d4f9ca43a273c1fe491b9bb002854eff6f9edeee32420b75a8c080378d74148103dd0229cb8c8482bf01");
    }
}
