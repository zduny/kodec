#[cfg(feature = "binary")]
pub mod binary;

#[cfg(feature = "json")]
pub mod json;

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

pub trait Encode {
    type Error;

    fn encode<W, T>(&self, writer: W, message: &T) -> Result<(), Self::Error>
    where
        W: Write,
        T: Serialize;
}

pub trait Decode {
    type Error;

    fn decode<R, T>(&self, data: R) -> Result<T, Self::Error>
    where
        R: Read,
        for<'de> T: Deserialize<'de>;
}
