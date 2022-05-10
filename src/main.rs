#[path = "nes_6502/nes_6502.rs"] mod nes_6502;
#[path = "nes_6502/status_flags.rs"] mod status_flags;
mod bus;
mod cpu;

use crate::bus::Bus;
use crate::bus::BusData;
use crate::nes_6502::NES6502;
use crate::cpu::CPU;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, time};

fn main() {

    let (to_bus_tx, to_bus_rx): (Sender<BusData>, Receiver<BusData>) = mpsc::channel();

    let cpu_to_bus_tx = to_bus_tx.clone();

    let (to_cpu_tx, to_cpu_rx): (Sender<BusData>, Receiver<BusData>) = mpsc::channel();

    let bus: Bus = Bus{ ram: [0; 64 * 1024], to_bus_rx: to_bus_rx, to_cpu_tx: to_cpu_tx};

    let mut cpu: NES6502 = CPU::new(cpu_to_bus_tx, to_cpu_rx);

    let ten_millis = time::Duration::from_millis(1000);

    thread::spawn(move || {
        loop {
            cpu.tick();
            thread::sleep(ten_millis);
        }
    });

    loop {
        bus.tick();
    }

}