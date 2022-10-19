use std::collections::BTreeMap;

use nom::IResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Bencode {
    Number(i64),
    ByteString(Vec<u8>),
    // Recursive data structure, exciting!
    List(Vec<Bencode>),
    Dict(BTreeMap<Vec<u8>, Bencode>),
}

pub fn parse_bencode(bencode_bytes: &[u8]) -> IResult<&[u8], Bencode> {
    todo!()
}

// bencode numbers are encoded as strings delimited by 'i' and 'e'
fn parse_number(bencode_bytes: &[u8]) -> IResult<&[u8], i64> {
    // delimited parses the bounds to a 'word' by being given 3 args
    delimited(
        tag("i"),
        // map_res allows us to return IResult and use closures
        map_res(
            // checking for delimiters
            is_not("e"),
            // bencode numbers are encoded as strings
            |bytes| String::from_utf8_lossy(bytes).parse::<i64>(),
        ),
        tag("e"),
    )(bencode_bytes)
}
