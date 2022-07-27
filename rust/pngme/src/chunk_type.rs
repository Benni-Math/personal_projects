// PNG file structure chunk types
// Based on PNG specs - http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
use std::convert::TryFrom;
use std::fmt;
use std::str::{FromStr, from_utf8};

use crate::{Error, Result};

// Creating a quick error type
#[derive(Debug)]
struct ChunkTypeError(String);

impl std::error::Error for ChunkTypeError {}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ChunkTypeError: {}", self.0)
    }
}

impl ChunkTypeError {
    fn new(s: &str) -> Box<ChunkTypeError> {
        Box::new(ChunkTypeError(s.to_string()))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.bytes.into_iter()
            .fold(true, |acc, c| acc && ChunkType::is_valid_byte(c))
            && self.bytes[2].is_ascii_uppercase()
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_alphabetic()
    }

    pub fn is_critical(&self) -> bool {
        self.bytes[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.bytes[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    
    fn try_from(value: [u8; 4]) -> Result<Self> {
        for byte in value {
            if !(byte as char).is_ascii_alphabetic() {
                // Unsure if this is proper error handling
                // Might need an Enum or something else
                return Err(ChunkTypeError::new("Failed TryFrom<[u8; 4]>. Byte array is not alphabetic."));
            }
        }    

        Ok(Self {
            bytes: value,
        })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Not the best practice to just unwrap here...
        let s = from_utf8(&self.bytes).unwrap();
        write!(f, "{}", s)
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {    
        // First, we check that everything is alphabetic    
        if let Some(_) = s.chars().find(|c| !c.is_ascii_alphabetic()) {
            return Err(ChunkTypeError::new("Failed FromStr. String is not ASCII alphabetic."));
        }

        // Next, we check the length of the string
        if s.len() != 4 { return Err(ChunkTypeError::new("Failed FromStr. String not of length 4.")); }

        // Finally, we use
        //      - as_bytes() to convert to &[u8]
        //      - implicitly deref and use try_into() to convert from slice to [u8; 4]
        //      - have a ? for error catching and unwrapping
        Ok(Self {
            bytes: s.as_bytes().try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }
    
    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }
    
    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116])
            .unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

