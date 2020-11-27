
pub struct Bus {
    pub ram: [u8; 64 * 1024]
}

impl Bus {

    pub fn write(&self, address: u16, data: u8) {
        println!("hi");
    }
    
    pub fn read(&self, address: u16, is_read_only: bool) {
        println!("hi");
    }

}

