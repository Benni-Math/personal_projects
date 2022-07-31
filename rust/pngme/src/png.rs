use std::{fmt, fs, path::Path, convert::TryFrom, str::FromStr};
use std::io::{BufReader, Read};

use crate::{Error, Result};
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;

#[derive(Debug)]
pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn header(&self) -> &[u8; 8] {
        &self.header
    }

    pub fn chunks(&self) -> &[Chunk] {
        self.chunks.as_slice()
    }

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Png {
            header: Png::STANDARD_HEADER,
            chunks,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        todo!()
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        todo!()
    }

    pub fn chunk_by_types(&self, chunk_type: &str) -> Option<&Chunk> {
        todo!()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::from(self.header);

        for chunk in self.chunks {
            bytes.append(&mut chunk.as_bytes());
        }

        bytes
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Png> {
        todo!()
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use crate::chunk::Chunk;
    use std::str::FromStr;
    use std::convert::TryFrom;

    fn testing_chunks() -> Vec<Chunk> {
        let mut chunks = Vec::new();

        chunks.push(chunk_from_strings("FrSt", "I am the first chunk").unwrap());
        chunks.push(chunk_from_strings("miDl", "I am another chunk").unwrap());
        chunks.push(chunk_from_strings("LASt", "I am the last chunk").unwrap());

        chunks
    }

    fn testing_png() -> Png {
        let chunks = testing_chunks();
        Png::from_chunks(chunks)
    }

    fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk> {
        use std::str::FromStr;

        let chunk_type = ChunkType::from_str(chunk_type)?;
        let data: Vec<u8> = data.bytes().collect();

        Ok(Chunk::new(chunk_type, data))
    }
}