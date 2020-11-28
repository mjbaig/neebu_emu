use addressing_modes::AddressingMode;


pub struct Instruction {
    name: str,
    op_code: OpCode,
    addressing_mode: AddressingMode,
    cycles: u8
}

pub enum OpCode {
    NONE
}

pub fn test2() {
    println!("test");
}