
use crate::nes_6502::NES6502;

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

pub fn execute_addressing_mode(cpu: &mut NES6502, addressing_mode: AddressingMode) {
    match addressing_mode {
        IMP => implied(cpu),
        IMM => immediate(cpu),
        ZPO => zero_page(cpu),
        ZPX => zero_page_with_x_offset(cpu),
        ZPY => zero_page_with_y_offset(cpu),
        REL => relative(cpu),
        ABS => absolute(cpu),
        ABX => absolute_with_x_offset(cpu),
        ABY => absolute_with_y_offset(cpu),
        IND => indirect(cpu),
        IZX => indirect_x(cpu),
        IZY => indirect_y(cpu),
        _ => 0
    };
}

fn implied(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

fn immediate(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

fn zero_page(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

fn zero_page_with_x_offset(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

fn zero_page_with_y_offset(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

fn relative(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

fn absolute(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

fn absolute_with_x_offset(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

fn absolute_with_y_offset(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

fn indirect(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

fn indirect_x(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

fn indirect_y(cpu: &mut NES6502) -> u8 {
    cpu.program_counter += 1;
    return 0;
}

