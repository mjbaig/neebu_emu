use crate::nes_6502::NES6502;

pub enum StatusFlags {
    C, // Carry
    Z, // Zero (when the result of an operation equals zero)
    I, // Disable Interrupts ()
    D, // Decimal Mode (not going to be used in NES)
    B, // Break
    U, // Unused
    V, // Overflow (signed variables)
    N // Negative (signed variables)
}

impl StatusFlags {
    pub fn value(&self) -> u8 {
        match *self {
            StatusFlags::C => 1 << 0,
            StatusFlags::Z => 1 << 1,
            StatusFlags::I => 1 << 2,
            StatusFlags::D => 1 << 3,
            StatusFlags::B => 1 << 4,
            StatusFlags::U => 1 << 5,
            StatusFlags::V => 1 << 6,
            StatusFlags::N => 1 << 7
        }
    }

    pub fn get_flag(&self) -> u8 {
        return self.value();
    } 

    pub fn set_flag(&self, cpu: &mut NES6502) {
        cpu.status_register = 0b00000001;
    }
}
