// #[path = "../bus.rs"] mod bus;
// #[path = "../cpu.rs"] mod cpu;
mod op_codes;


use crate::bus::Bus;
use crate::cpu::CPU;


pub struct OLC6502 {
    pub bus: Bus
}


impl CPU for OLC6502 {

    fn new(bus: Bus) -> OLC6502 {
        OLC6502{bus: bus}
    }

    fn read(&self, address: u16, is_read_only: bool) {
        &self.bus.read(address, false);
    }

    fn write(&self, address: u16, data: u8) {
        &self.bus.write(address, data);
    }
}


