use crate::{Instruction, InstructionCode, DecodingError, utils};
use num_traits::ToPrimitive;
use num_bigint::ToBigInt;

#[derive(Debug)]
pub struct Copy {
    pub index: usize
}

impl Instruction for Copy {
    fn to_assembly(&self) -> String {
        format!("copy {}", self.index).into()
    }

    fn code(&self) -> InstructionCode {
        InstructionCode::Copy
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_vlq_argument(InstructionCode::Copy, &self.index.to_bigint().unwrap())
    }

    fn inputs_count(&self) -> usize {
        0
    }

    fn outputs_count(&self) -> usize {
        1
    }
}

impl Copy {
    pub fn new(index: usize) -> Self {
        Self { index }
    }

    pub fn decode(bytes: &[u8]) -> Result<(Copy, usize), DecodingError> {
        let (value, len) = utils::decode_with_vlq_argument(InstructionCode::Copy, bytes)?;
        let index = value.to_usize().ok_or(DecodingError::ConstantTooLong)?;
        Ok((Copy { index }, len))
    }
}
