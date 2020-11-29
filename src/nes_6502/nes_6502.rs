mod addressing_modes;

use crate::bus::BusData;
use crate::bus::ReadWrite;
use crate::cpu::CPU;
use crate::status_flags::StatusFlags;
use std::sync::mpsc::{Sender, Receiver};
use addressing_modes::AddressingMode;



pub struct NES6502 {
    accumulator_register: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    program_counter: u16,
    pub status_register: u8,
    fetched: u8,
    temp: u16,
    cycles: u8,
    absolute_address: u16,
    relative_address: u16,
    op_code: u8,
    clock_count: u32,
    pub to_bus_tx: Sender<BusData>,
    pub to_cpu_rx: Receiver<BusData>
}

impl NES6502 {
    fn fetch() {
        println!("called fetch");
    }
}


impl CPU for NES6502 {

    fn new(to_bus_tx: Sender<BusData>, to_cpu_rx: Receiver<BusData>) -> NES6502 {
        NES6502 {
            accumulator_register: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            stack_pointer: 0x00,
            program_counter: 0x000,
            status_register: 0x00,
            fetched: 0x00,
            temp: 0x0000,
            cycles: 0,
            absolute_address: 0x0000,
            relative_address: 0x00,
            op_code: 0x00,
            clock_count: 0,
            to_bus_tx: to_bus_tx,
            to_cpu_rx: to_cpu_rx
        }
    }

    fn tick(&mut self){
        let bus_data = BusData{method: ReadWrite::WRITE, address: self.program_counter, data: Some(0x01)};
        self.to_bus_tx.send(bus_data).unwrap();
        addressing_modes::execute_addressing_mode(self, AddressingMode::IMP);
        println!("tick");
    }

    fn reset(&mut self) {
        println!("called reset");
    }

    fn interupt_request(&mut self) {
        println!("called interupt_request");
    }

    fn non_maskable_interrupt_request(&mut self) {
        println!("called non_maskable_interrupt_request");
    }
}


