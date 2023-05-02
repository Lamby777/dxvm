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

	let mut state = DxVMState {
		cursor:	&mut 0,
		stack:	&mut vec![],
	};

	loop {
		// Run instruction
		let res = interpret(
			bytecode,
			&mut state,
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
	state:		&mut DxVMState,
) -> InstructionResult {
	let DxVMState { cursor, stack } = state;

	let opcode = bytecode[**cursor];

	let instruction: INSTR = (opcode as u32)
		.try_into().expect("Failed to parse opcode into instruction!");

	// Number of bytes to move forward after this instruction
	let mut skip: Option<u64> = Some(1);
	
	match instruction {
		INSTR::Exit		=> {
			// write a function to interpret dynamic stuff later
			return InstructionResult::Exit(bytecode[**cursor + 1] as i64);
		},

		INSTR::Push		=> {
			// same "do this later" ^^^
			let val = bytecode[**cursor + 1];
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
		**cursor += v as usize;
	};

	InstructionResult::Continue
}

pub struct DxVMState<'a> {
	cursor:	&'a mut usize,
	stack:	&'a mut DxVMStack,
}
