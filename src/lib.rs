/*
* "some helpful information about the package"
* - Cherry 4/27/2023
*/

use instructions::INSTR;
use libdx::Result;

mod instructions;

#[cfg(test)]
mod tests;

pub fn execute_binary(bytecode: &[u64]) -> Result<i64> {
	let cursor = 0usize;
	let exit_code;

	loop {
		let opcode = bytecode[cursor];

		let instruction: INSTR = (opcode as u32).try_into()?;

		match instruction {
			INSTR::Exit		=> {
				// write a function to interpret dynamic stuff later
				exit_code = bytecode[cursor + 1] as i64;
				break;
			},

			INSTR::Push		=> todo!(),
			INSTR::Pop		=> todo!(),
			INSTR::Incr		=> todo!(),
			INSTR::Decr		=> todo!(),
		}
	}

	Ok(exit_code)
}
