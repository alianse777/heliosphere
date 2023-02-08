//! Utility functions
use serde::{Deserialize, Deserializer, Serializer};
use std::fmt::{Display, LowerHex};

/// Ser/De number as hex (without 0x prefix)
pub mod as_hex_number {
    use super::*;

    /// Serialize
    pub fn serialize<S, T>(val: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: LowerHex,
    {
        serializer.serialize_str(&format!("{val:#x}"))
    }

    /// Deserialize (strip 0x if present)
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: TryFrom<i128>,
        <T as TryFrom<i128>>::Error: Display,
    {
        let s = String::deserialize(deserializer)?;
        let val = i128::from_str_radix(s.trim_start_matches("0x"), 16)
            .map_err(serde::de::Error::custom)?;
        val.try_into().map_err(serde::de::Error::custom)
    }
}

/// Ser/De u8 buffer as hex string
pub mod as_hex_buffer {
    use super::*;

    /// Serialize
    pub fn serialize<S>(buf: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(buf)))
    }

    /// Deserialize (strip 0x if present)
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        hex::decode(s.trim_start_matches("0x")).map_err(serde::de::Error::custom)
    }
}

/// Ser/De u8 array as hex string
pub mod as_hex_array {
    use super::*;

    /// Serialize
    pub fn serialize<S>(buf: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(buf))
    }

    /// Deserialize (strip 0x if present)
    pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let buf = hex::decode(s.trim_start_matches("0x")).map_err(serde::de::Error::custom)?;
        buf.try_into()
            .map_err(|_| serde::de::Error::custom("invalid size"))
    }
}

/// Ser/De address as hex string
pub mod as_hex_address {
    use crate::Address;

    use super::*;

    /// Serialize
    pub fn serialize<S>(address: &Address, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(address.as_bytes()))
    }

    /// Deserialize (strip 0x if present)
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(s.trim_start_matches("0x")).map_err(serde::de::Error::custom)?;
        Address::new(
            bytes
                .try_into()
                .map_err(|_| serde::de::Error::custom("invalid len"))?,
        )
        .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    #[repr(transparent)]
    struct Val(#[serde(with = "as_hex_number")] u64);

    #[test]
    fn as_hex_number() {
        assert_eq!(serde_json::to_string(&Val(0)).unwrap(), "\"0x0\"");
        assert_eq!(serde_json::to_string(&Val(65)).unwrap(), "\"0x41\"");
        assert_eq!(serde_json::to_string(&Val(1024)).unwrap(), "\"0x400\"");
    }

    #[derive(Serialize, Deserialize)]
    #[repr(transparent)]
    struct Buffer(#[serde(with = "as_hex_buffer")] Vec<u8>);

    #[test]
    fn test_hex_buf() {
        assert_eq!(serde_json::to_string(&Buffer(vec![])).unwrap(), "\"0x\"");
        assert_eq!(serde_json::to_string(&Buffer(vec![0])).unwrap(), "\"0x00\"");
        assert_eq!(
            serde_json::to_string(&Buffer(vec![b'A'])).unwrap(),
            "\"0x41\""
        );
    }
}
