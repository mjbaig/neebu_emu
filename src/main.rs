#[path = "olc6502/olc6502.rs"] mod olc6502;
#[path = "olc6502/status_flags.rs"] mod status_flags;
mod bus;
mod cpu;

use crate::bus::Bus;
use crate::olc6502::OLC6502;
use crate::cpu::CPU;

fn main() {

    let mut cpu_bus: Bus = Bus{ ram: [0; 64 * 1024]};

    let mut cpu: OLC6502 = CPU::new(&mut cpu_bus);

    cpu.read(0, false);
    cpu.write(1,1);

    println!("{:?}", cpu_bus.ram[1]);

}
