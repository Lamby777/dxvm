/*
* "some helpful information about the package"
* - Cherry 4/27/2023
*/

use instructions::{INSTR, InstructionResult};
use libdx::Result;

mod instructions;

#[cfg(test)]
mod tests;

pub type DxVMStack = Vec<u64>;

pub fn execute_binary(bytecode: &[u64]) -> Result<i64> {
	let exit_code;
	let mut cursor = 0usize;
	let mut stack1: DxVMStack = vec![];

	loop {
		// Run instruction
		let res = interpret(
			bytecode,
			&mut cursor,
			&mut stack1
		);

		match res {
			InstructionResult::Exit(v) => {
				exit_code = v;
				break;
			}

			_ => ()
		}
	}

	Ok(exit_code)
}

pub fn interpret(
	bytecode:	&[u64],
	cursor:		&mut usize,
	stack:		&mut DxVMStack
) -> InstructionResult {
	let opcode = bytecode[*cursor];

	let instruction: INSTR = (opcode as u32)
		.try_into().expect("Failed to parse opcode into instruction!");

	// Number of bytes to move forward after this instruction
	let mut skip: Option<u64> = Some(1);
	
	match instruction {
		INSTR::Exit		=> {
			// write a function to interpret dynamic stuff later
			return InstructionResult::Exit(bytecode[*cursor + 1] as i64);
		},

		INSTR::Push		=> {
			// same "do this later" ^^^
			let val = bytecode[*cursor + 1];
			stack.push(val);

			skip = Some(2);
		},

		INSTR::Pop		=> {
			// add some way to read the value later ("virtual" cpu registers???)
			stack.pop();
		},

		INSTR::Incr		=> todo!(),
		INSTR::Decr		=> todo!(),
	}

	if let Some(v) = skip {
		*cursor += v as usize;
	};

	InstructionResult::Continue
}
