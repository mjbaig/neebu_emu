#[path = "olc6502/olc6502.rs"] mod olc6502;
mod bus;
mod cpu;

use crate::bus::Bus;
use crate::olc6502::OLC6502;
use crate::cpu::CPU;

fn main() {

    let bus: Bus = Bus{ ram: [0; 64 * 1024]};

    let cpu: OLC6502 = CPU::new(bus);

    cpu.read(0, false);
    cpu.write(0,0);

}
