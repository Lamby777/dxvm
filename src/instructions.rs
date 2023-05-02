/*
* Parse instructions from a binary
*/

use num_derive::FromPrimitive;
use crate::classes::OpcodeLookupError;

impl TryFrom<u32> for INSTR {
	type Error = OpcodeLookupError;
	
	fn try_from(opcode: u32) -> Result<Self, Self::Error> {
		let option = num::FromPrimitive::from_u32(opcode);
		
		option.ok_or_else(|| OpcodeLookupError)
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
