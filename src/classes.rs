/*
* Structs, enums, maybe impls??? etc.
*/

use std::{error::Error, fmt};

// type aliases
pub type DxVMStack = Vec<u64>;


// other more complicated stuff

#[derive(Debug)]
pub struct OpcodeLookupError;
impl Error for OpcodeLookupError {}

impl fmt::Display for OpcodeLookupError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Oh no, something bad went down")
	}
}

pub enum InstructionResult {
	Continue,
	Exit(i64),
}

pub struct DxVMState {
	pub cursor:	usize,
	pub stack:	DxVMStack,
}
