use serde::{Serialize, Deserialize};
use std::io::{Read, Write};

use crate::{Encode, Decode};

pub struct Codec {}

impl Encode for Codec {
    type Error = serde_json::Error;

    fn encode<W, T>(writer: W, message: &T) -> Result<(), serde_json::Error>
    where
        W: Write, T: Serialize {
        serde_json::to_writer(writer, message)
    }
}

impl Decode for Codec {
    type Error = serde_json::Error;
    
    fn decode<R, T>(reader: R) -> Result<T, serde_json::Error>
    where 
    R: Read,
    for<'de> T: Deserialize<'de>
    {
        serde_json::from_reader(reader)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Encode, Decode, json};

    #[test]
    fn test_json() {
        let message: (i16, String) = (10, "Hello World!".to_string());
        let mut buffer = vec![];
        json::Codec::encode(&mut buffer, &message).unwrap();
        let decoded = json::Codec::decode(&buffer[..]).unwrap();
        assert_eq!(message, decoded);
    }
}
