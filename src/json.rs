use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

use crate::{Decode, Encode};

/// JSON codec.
#[derive(Debug, Clone, Copy, Default)]
pub struct Codec {}

impl Encode for Codec {
    type Error = serde_json::Error;

    fn encode<W, T>(&self, writer: W, message: &T) -> Result<(), serde_json::Error>
    where
        W: Write,
        T: Serialize,
    {
        serde_json::to_writer(writer, message)
    }
}

impl Decode for Codec {
    type Error = serde_json::Error;

    fn decode<R, T>(&self, reader: R) -> Result<T, serde_json::Error>
    where
        R: Read,
        for<'de> T: Deserialize<'de>,
    {
        serde_json::from_reader(reader)
    }
}

#[cfg(test)]
mod tests {
    use crate::json::Codec;
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
