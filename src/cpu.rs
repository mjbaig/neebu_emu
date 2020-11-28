// mod bus;
use crate::bus::Bus;
use crate::bus::BusData;
use std::sync::mpsc::{Sender, Receiver};


pub trait CPU {
    fn new(to_bus_tx: Sender<BusData>, to_cpu_rx: Receiver<BusData>) -> Self;
    fn tick(&mut self);
    fn reset(&mut self);
    fn interupt_request(&mut self);
    fn non_maskable_interrupt_request(&mut self);
}