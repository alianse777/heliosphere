//! Universal address representation
use crate::error::Error;
use alloc::string::String;
use core::fmt::{Debug, Display};
use core::str::FromStr;
use serde::{Deserialize, Serialize};
use zerocopy::AsBytes;

/// Account address struct
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Address([u8; 21]);

impl Address {
    /// Construct new address from bytes (expected with 0x41 prefix)
    pub fn new(bytes: [u8; 21]) -> Result<Self, Error> {
        if bytes[0] == 0x41 {
            Ok(Self(bytes))
        } else {
            Err(Error::InvalidAddress)
        }
    }

    /// Get base58 representation
    pub fn as_base58(&self) -> alloc::string::String {
        bs58::encode(&self.0).with_check().into_string()
    }

    /// Get hex representation
    pub fn as_hex(&self) -> alloc::string::String {
        hex::encode(self.0)
    }

    /// Get raw address bytes (including 0x41 prefix)
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Parse address from base58 or hex string
impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = bs58::decode(s)
            .with_check(None)
            .into_vec()
            .or_else(|_| hex::decode(s))
            .map_err(|_| Error::InvalidAddress)?;
        Ok(Self(bytes.try_into().map_err(|_| Error::InvalidAddress)?))
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_base58())
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_base58())
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.as_base58())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "ethabi_compat")]
impl From<ethabi::Address> for Address {
    fn from(address: ethabi::Address) -> Self {
        let mut buf = [0x41; 21];
        buf[1..].copy_from_slice(address.as_bytes());
        Self(buf)
    }
}

#[cfg(feature = "ethabi_compat")]
impl From<Address> for ethabi::Address {
    fn from(address: Address) -> Self {
        Self(address.0[1..].try_into().expect("Always 20 bytes"))
    }
}

impl From<alloy_primitives::Address> for Address {
    fn from(address: alloy_primitives::Address) -> Self {
        let mut buf = [0x41; 21];
        buf[1..].copy_from_slice(address.as_bytes());
        Self(buf)
    }
}

impl From<Address> for alloy_primitives::Address {
    fn from(address: Address) -> Self {
        Self(address.0[1..].try_into().expect("Always 20 bytes"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_address_full() {
        let hex = "418840E6C55B9ADA326D211D818C34A994AECED808";
        let b58 = "TNPeeaaFB7K9cmo4uQpcU32zGK8G1NYqeL";
        let bytes = hex::decode(hex).unwrap();

        let a1 = Address::new(bytes.try_into().unwrap()).expect("Address::new");
        let a2: Address = b58.parse().expect("b58 parse");
        let a3: Address = hex.parse().expect("hex parse");

        assert!(a1 == a2 && a2 == a3, "address mismatch");
        assert_eq!(a1.as_base58(), b58, "b58 mismatch");
        assert_eq!(a1.as_hex().to_ascii_uppercase(), hex, "hex mismatch");
    }
}
