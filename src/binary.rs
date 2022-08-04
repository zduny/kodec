use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

use crate::{Decode, Encode};

#[derive(Debug, Clone, Copy, Default)]
pub struct Codec {}

impl Encode for Codec {
    type Error = bincode::Error;

    fn encode<W, T>(&self, writer: W, message: &T) -> Result<(), bincode::Error>
    where
        W: Write,
        T: Serialize,
    {
        bincode::serialize_into(writer, message)
    }
}

impl Decode for Codec {
    type Error = bincode::Error;

    fn decode<R, T>(&self, reader: R) -> Result<T, bincode::Error>
    where
        R: Read,
        for<'de> T: Deserialize<'de>,
    {
        bincode::deserialize_from(reader)
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::Codec;
    use crate::{Decode, Encode};

    #[test]
    fn test_json() {
        let message: (i16, String) = (10, "Hello World!".to_string());
        let mut buffer = vec![];
        let codec = Codec::default();
        codec.encode(&mut buffer, &message).unwrap();
        let decoded = codec.decode(&buffer[..]).unwrap();
        assert_eq!(message, decoded);
    }
}
