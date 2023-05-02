use crate::*;
use instructions::INSTR;

#[cfg(test)]
mod parsing {
	use super::*;

	#[test]
	fn opcode_to_instr() -> Result<()> {
		// Exit instruction
		let exit_instr: INSTR = 0.try_into()?;
		assert_eq!(exit_instr, INSTR::Exit);

		// Invalid instruction
		let invalid_instr: Result<INSTR, _> = 192_168.try_into();
		assert!(invalid_instr.is_err());

		Ok(())
	}
}

#[cfg(test)]
mod execution {
	use super::*;
	
	#[test]
	fn exit_immediately() -> Result<()> {
		for n in [0, 1, 12345] {
			let res = execute_binary(
				&[
					0x00000000, // EXIT
					n, // exit code
				]
			)?;

			assert_eq!(res, n as i64);
		}

		Ok(())
	}
}
