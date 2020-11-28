
use crate::olc6502::OLC6502;

pub enum AddressingMode {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    REL,
    ABS,
    ABX,
    ABY,
    IND,
    IZX,
    IZY
}

pub fn execute_addressing_mode(cpu: &mut OLC6502, addressing_mode: AddressingMode) {
    match addressing_mode {
        IMP => implied(cpu),
        _ => 0
    };
}

fn implied(cpu: &mut OLC6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}