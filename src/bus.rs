use std::sync::mpsc::{Sender, Receiver};

pub struct Bus {
    pub ram: [u8; 64 * 1024],
    pub to_bus_rx: Receiver<BusData>,
    pub to_cpu_tx: Sender<BusData>
}

impl Bus {

    pub fn write(&self, address: u16, data: u8) {
        println!("hi");
    }
    
    pub fn read(&self, address: u16, is_read_only: bool) {
        println!("hi");
    }

}

pub struct BusData {
    method: ReadWrite,
    address: u16,
    data: Option<u8>
}

pub enum ReadWrite {
    READ,
    WRITE
}
