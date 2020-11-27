// mod bus;
use crate::bus::Bus;

pub trait CPU {
    fn new(bus: Bus) -> Self;
    fn read(&self, address: u16, is_read_only: bool);
    fn write(&self, address: u16, data: u8);
}