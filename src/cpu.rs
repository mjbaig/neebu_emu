// mod bus;
use crate::bus::Bus;

pub trait CPU<'a> {
    fn new(bus: &'a mut Bus) -> Self;
    fn read(&mut self, address: u16, is_read_only: bool);
    fn write(&mut self, address: u16, data: u8);
}