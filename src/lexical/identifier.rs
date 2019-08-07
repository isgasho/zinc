//!
//! The identifier lexeme.
//!

use std::convert::TryFrom;

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::Keyword;

#[derive(Debug, Serialize)]
pub struct Identifier(String);

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "is keyword: {:?}", _0)]
    IsKeyword(Keyword),
}

impl Identifier {
    pub fn can_start_with(byte: u8) -> bool {
        byte.is_ascii_alphabetic() || byte == b'_'
    }

    pub fn can_contain_since_index_1(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_'
    }
}

impl TryFrom<&[u8]> for Identifier {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if let Ok(keyword) = Keyword::try_from(bytes) {
            return Err(Error::IsKeyword(keyword));
        }

        Ok(Self {
            0: String::from_utf8_lossy(bytes).to_string(),
        })
    }
}
