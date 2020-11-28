
pub struct Bus {
    pub ram: [u8; 64 * 1024]
}

impl Bus {

    pub fn write(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }
    
    pub fn read(&self, address: u16, is_read_only: bool) {
        println!("hi");
    }

}

