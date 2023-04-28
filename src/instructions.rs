/*
* Parse instructions from a binary
*/

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct OpcodeLookupError;
impl Error for OpcodeLookupError {}

impl fmt::Display for OpcodeLookupError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Oh no, something bad went down")
	}
}


#[derive(Clone)]
pub enum Instruction {
	Exit,
	Push,
	Pop,
}

impl TryFrom<u64> for Instruction {
	type Error = OpcodeLookupError;

	fn try_from(opcode: u64) -> Result<Self, OpcodeLookupError> {
		match opcode {
			0x0000000000000000	=> Ok(Self::Exit),
			0x0000000000000001	=> Ok(Self::Push),
			0x0000000000000002	=> Ok(Self::Pop),
			_					=> Err(OpcodeLookupError),
		}
	}
}
