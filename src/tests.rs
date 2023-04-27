use crate::*;

#[test]
fn do_nothing() -> Result<()> {
	let res = execute_binary(
		&[]
	)?;

	assert_eq!(res, 0);

	Ok(())
}
