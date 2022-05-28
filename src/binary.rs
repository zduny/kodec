use serde::{Serialize, Deserialize};
use std::io::{Read, Write};

use crate::{Encode, Decode};

pub struct Codec {}

impl Encode for Codec {
    type Error = bincode::Error;

    fn encode<W, T>(writer: W, message: &T) -> Result<(), bincode::Error>
    where
        W: Write, T: Serialize {
        bincode::serialize_into(writer, message)
    }
}

impl Decode for Codec {
    type Error = bincode::Error;
    
    fn decode<R, T>(reader: R) -> Result<T, bincode::Error>
    where 
    R: Read,
    for<'de> T: Deserialize<'de>
    {
        bincode::deserialize_from(reader)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Encode, Decode, binary};

    #[test]
    fn test_binary() {
        let message: (i16, String) = (10, "Hello World!".to_string());
        let mut buffer = vec![];
        binary::Codec::encode(&mut buffer, &message).unwrap();
        let decoded = binary::Codec::decode(&buffer[..]).unwrap();
        assert_eq!(message, decoded);
    }
}
