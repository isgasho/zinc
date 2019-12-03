//!
//! The semantic analyzer array value element error.
//!

use failure::Fail;

use crate::semantic::Type;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "pushing a value of type '{}', but expected '{}'", _0, _1)]
    PushingInvalidType(Type, Type),
}
