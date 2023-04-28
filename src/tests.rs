use crate::*;
use instructions::INSTR;

#[test]
fn do_nothing() -> Result<()> {
	let res = execute_binary(
		&[]
	)?;

	assert_eq!(res, 0);

	Ok(())
}

#[test]
fn opcode_from() -> Result<()> {
	let exit_instr: INSTR = 0.try_into()?;
	assert_eq!(exit_instr, INSTR::Exit);

	Ok(())
}
