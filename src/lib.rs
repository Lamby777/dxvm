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
	let mut cursor = 0usize;
	let exit_code;

	loop {
		let opcode = bytecode[cursor];

		let instruction: INSTR = (opcode as u32).try_into()?;

		// Number of bytes to move forward after this instruction
		let mut skip: Option<u64> = Some(1);
		
		match instruction {
			INSTR::Exit		=> {
				// write a function to interpret dynamic stuff later
				exit_code = bytecode[cursor + 1] as i64;
				break;
			},

			INSTR::Push		=> todo!(),
			INSTR::Pop		=> todo!(),
			INSTR::Incr		=> todo!(),
			INSTR::Decr		=> (), // prevent "unused code" warnings
		}

		if let Some(v) = skip {
			cursor += v as usize;
		}
	}

	Ok(exit_code)
}
