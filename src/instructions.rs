use phf::phf_map;

#[derive(Clone)]
pub enum Instruction {
	Exit,
}

static OPCODES: phf::Map<u64, Instruction> = phf_map! {
	0x0000000000000000u64	=> Instruction::Exit,
};

pub fn opcode_to_instruction(opcode: u64) -> Option<Instruction> {
	OPCODES.get(&opcode).cloned()
}
