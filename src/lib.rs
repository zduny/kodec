//! Simple crate defining common interface for encoders and decoders.
//!
//! Also provides JSON and binary (bincode) codec implementations
//! that can be enabled with `json` and/or `binary` features.

#[cfg(feature = "binary")]
pub mod binary;

#[cfg(feature = "json")]
pub mod json;

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

/// Trait for encoders.
pub trait Encode {
    type Error;

    /// Encode message into writer.
    fn encode<W, T>(&self, writer: W, message: &T) -> Result<(), Self::Error>
    where
        W: Write,
        T: Serialize;
}

/// Trait for decoders.
pub trait Decode {
    type Error;

    /// Decode message from reader.
    fn decode<R, T>(&self, data: R) -> Result<T, Self::Error>
    where
        R: Read,
        for<'de> T: Deserialize<'de>;
}
