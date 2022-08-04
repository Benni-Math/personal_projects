use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::{Error, Result};
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let input = args.path.as_path();
    let png_bytes = fs::read(input)?;
    let mut png = Png::try_from(png_bytes.as_ref())?;

    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let data = Vec::from(args.msg.as_bytes());

    let chunk = Chunk::new(chunk_type, data);

    png.append_chunk(chunk);

    let png_file = png.as_bytes();

    if let Some(output) = args.output {
        // write to new output file
        fs::write(output, png_file.as_slice())?;
    } else {
        // write to the original file
        fs::write(input, png_file)?;
    }
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let input = args.path.as_path();
    let png_bytes = fs::read(input)?;
    let png = Png::try_from(png_bytes.as_ref())?;

    let chunk = png.chunk_by_type(&args.chunk_type);
    match chunk {
        Some(chunk) => println!("Message found: {}", chunk.data_as_string()?),
        None => println!("Message not found."),
    }

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let input = args.path.as_path();
    let png_bytes = fs::read(input)?;
    let mut png = Png::try_from(png_bytes.as_ref())?;

    png.remove_chunk(&args.chunk_type)?;

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let input = args.path.as_path();
    let png_bytes = fs::read(input)?;
    let png = Png::try_from(png_bytes.as_ref())?;

    println!("{}", png);

    Ok(())
}