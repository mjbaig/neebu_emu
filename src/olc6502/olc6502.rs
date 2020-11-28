use crate::bus::Bus;
use crate::cpu::CPU;
use crate::status_flags::StatusFlags;


pub struct OLC6502<'a> {
    pub bus: &'a mut Bus,
    accumulator_register: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    program_counter: u16,
    status_register: u8
}


impl<'a> CPU<'a> for OLC6502<'a> {

    fn new(bus: &'a mut Bus) -> OLC6502<'a> {
        OLC6502{
            bus: bus,
            accumulator_register: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            stack_pointer: 0x00,
            program_counter: 0x000,
            status_register: 0x00 
        }
    }

    fn read(&mut self, address: u16, is_read_only: bool) {
        self.x_register = self.x_register + 1;
        println!("{}", self.x_register);
        &self.bus.read(address, false);
    }

    fn write(&mut self, address: u16, data: u8) {
        &self.bus.write(address, data);
    }
}


