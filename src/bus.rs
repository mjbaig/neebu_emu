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

    pub fn tick(&self) {
        let bus_data = self.to_bus_rx.recv().unwrap();
        let data = bus_data.data;
        let address = bus_data.address;
        match data {
            Option::Some(_) => println!("address:{} data:{:?}", address, data.unwrap()),
            Option::None => println!("no data was supplied in this request")
        }
    }

}

pub struct BusData {
    pub method: ReadWrite,
    pub address: u16,
    pub data: Option<u8>
}

pub enum ReadWrite {
    READ,
    WRITE
}
