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


#[derive(Clone, Debug, PartialEq)]
pub enum INSTR {
	Exit,
	Push,
	Pop,
}

impl TryFrom<u64> for INSTR {
	type Error = OpcodeLookupError;

	fn try_from(opcode: u64) -> Result<Self, Self::Error> {
		match opcode {
			0x00000000	=> Ok(Self::Exit),
			0x00000001	=> Ok(Self::Push),
			0x00000002	=> Ok(Self::Pop),
			_			=> Err(OpcodeLookupError),
		}
	}
}
