use crate::bus::BusData;
use crate::bus::ReadWrite;
use crate::cpu::CPU;
use crate::status_flags::StatusFlags;
use std::sync::mpsc::{Sender, Receiver};



pub struct OLC6502 {
    accumulator_register: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    program_counter: u16,
    status_register: u8,
    pub to_bus_tx: Sender<BusData>,
    pub to_cpu_rx: Receiver<BusData>
}


impl CPU for OLC6502 {

    fn new(to_bus_tx: Sender<BusData>, to_cpu_rx: Receiver<BusData>) -> OLC6502 {
        OLC6502{
            accumulator_register: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            stack_pointer: 0x00,
            program_counter: 0x000,
            status_register: 0x00,
            to_bus_tx: to_bus_tx,
            to_cpu_rx: to_cpu_rx
        }
    }

    fn tick(&mut self){
        let bus_data = BusData{method: ReadWrite::WRITE, address: 0x00, data: Some(0x01)};
        self.to_bus_tx.send(bus_data).unwrap();
        println!("tick");
    }
}


