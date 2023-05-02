/*
* Parse instructions from a binary
*/

use std::{error::Error, fmt};
use num_derive::FromPrimitive;

#[derive(Debug)]
pub struct OpcodeLookupError;
impl Error for OpcodeLookupError {}

impl fmt::Display for OpcodeLookupError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Oh no, something bad went down")
	}
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, FromPrimitive)]
pub enum INSTR {
	// Core
	Exit				= 0x00000000,

	// Registers / Memory
	Push				= 0x00010000,
	Pop					= 0x00010001,
	
	// For Convenience
	Incr				= 0x00020000,
	Decr				= 0x00020001,
}

impl TryFrom<u32> for INSTR {
	type Error = OpcodeLookupError;
	
	fn try_from(opcode: u32) -> Result<Self, Self::Error> {
		let option = num::FromPrimitive::from_u32(opcode);
		
		option.ok_or_else(|| OpcodeLookupError)
	}
}
