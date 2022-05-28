#[cfg(feature = "binary")]
pub mod binary;

#[cfg(feature = "json")]
pub mod json;

use std::io::{Write, Read};

use serde::{Serialize, Deserialize};

pub trait Encode {
    type Error;

    fn encode<W, T>(writer: W, message: &T) -> Result<(), Self::Error>
    where
        W: Write,
        T: Serialize;
}

pub trait Decode {
    type Error;

    fn decode<R, T>(data: R) -> Result<T, Self::Error>
    where
        R: Read,
        for<'de> T: Deserialize<'de>;
}
