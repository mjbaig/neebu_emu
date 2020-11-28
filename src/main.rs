#[path = "olc6502/olc6502.rs"] mod olc6502;
#[path = "olc6502/status_flags.rs"] mod status_flags;
mod bus;
mod cpu;

use crate::bus::Bus;
use crate::bus::BusData;
use crate::olc6502::OLC6502;
use crate::cpu::CPU;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, time};

fn main() {

    let (to_bus_tx, to_bus_rx): (Sender<BusData>, Receiver<BusData>) = mpsc::channel();
    let cpu_to_bus_tx = to_bus_tx.clone();

    let (to_cpu_tx, to_cpu_rx): (Sender<BusData>, Receiver<BusData>) = mpsc::channel();

    let mut bus: Bus = Bus{ ram: [0; 64 * 1024], to_bus_rx: to_bus_rx, to_cpu_tx: to_cpu_tx};

    let mut cpu: OLC6502 = CPU::new(cpu_to_bus_tx, to_cpu_rx);

    let ten_millis = time::Duration::from_millis(1000);
    let now = time::Instant::now();

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