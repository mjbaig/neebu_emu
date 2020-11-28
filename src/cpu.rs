// mod bus;
use crate::bus::Bus;
use crate::bus::BusData;
use std::sync::mpsc::{Sender, Receiver};


pub trait CPU {
    fn new(bus: Bus, to_bus_tx: Sender<BusData>, to_cpu_rx: Receiver<BusData>) -> Self;
    fn read(&mut self, address: u16, is_read_only: bool);
    fn write(&mut self, address: u16, data: u8);
}