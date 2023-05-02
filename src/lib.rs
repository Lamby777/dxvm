/*
* "some helpful information about the package"
* - Cherry 4/27/2023
*/

use instructions::INSTR;
use libdx::Result;

mod instructions;

#[cfg(test)]
mod tests;

pub fn execute_binary(program_bytes: &[u64]) -> Result<i64> {
	let bytecode = program_bytes.iter();

	let exit_code = 0;

	for &opcode in bytecode {
		let instruction: INSTR = (opcode as u32).try_into()?;

		match instruction {
			INSTR::Exit		=> todo!(),
			INSTR::Push		=> todo!(),
			INSTR::Pop		=> todo!(),
			INSTR::Store	=> todo!(),
			INSTR::Deref	=> todo!(),
			INSTR::Incr		=> todo!(),
			INSTR::Decr		=> todo!(),
			INSTR::NOT		=> todo!(),
			INSTR::AND		=> todo!(),
			INSTR::OR		=> todo!(),
			INSTR::Add		=> todo!(),
			INSTR::Sub		=> todo!(),
			INSTR::Mul		=> todo!(),
			INSTR::Div		=> todo!(),
		}
	}

	Ok(exit_code)
}
