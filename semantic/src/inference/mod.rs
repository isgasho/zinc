//!
//! Inference.
//!

mod error;

pub use self::error::Error;

use num_bigint::BigInt;
use num_traits::Num;

use parser::IntegerLiteral;

pub fn integer_literal(literal: &IntegerLiteral) -> Result<(String, usize), Error> {
    let (string, base) = match literal {
        IntegerLiteral::Decimal { value } => (value, crate::BASE_DECIMAL as u32),
        IntegerLiteral::Hexadecimal { value } => (value, crate::BASE_HEXADECIMAL as u32),
    };

    let number = BigInt::from_str_radix(string, base).expect("Always valid");
    let mut bitlength = crate::BITLENGTH_BYTE;
    let mut exponent = BigInt::from(crate::MAX_VALUE_BYTE);
    while number >= exponent {
        if bitlength == crate::BITLENGTH_MAX_INT {
            exponent *= 64;
            bitlength += crate::BITLENGTH_FIELD - crate::BITLENGTH_MAX_INT;
        } else if bitlength == crate::BITLENGTH_FIELD {
            return Err(Error::LiteralTooLarge(bitlength));
        } else {
            exponent *= crate::MAX_VALUE_BYTE;
            bitlength += crate::BITLENGTH_BYTE;
        }
    }

    Ok((number.to_string(), bitlength))
}
