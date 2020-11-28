// mod bus;
use crate::bus::Bus;
use crate::bus::BusData;
use std::sync::mpsc::{Sender, Receiver};


pub trait CPU {
    fn new(to_bus_tx: Sender<BusData>, to_cpu_rx: Receiver<BusData>) -> Self;
    fn clock(&mut self);
}