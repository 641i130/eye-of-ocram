use crate::reader::WldReader;

pub struct World {
    pub file_name: String,
    pub version: u8, // first byte of file
    pub name_len: usize, // the byte at someplace
    pub name: String, // length is byte before i think...
}
impl World {
    pub fn new(wldfile:&String) -> World { 
        World {
            file_name: wldfile.to_string(),
            version: 0, // int 32 
            name_len: 0,
            name: "".to_string(),
        }
    }
    pub fn read_wld(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut wldp = WldReader::new(&self.file_name);
        Ok(())
    }
    pub fn pretty_print(self) {
        println!("\nWorld version : {:?}",self.version);
        println!("World name is 0x{:02X} chars long.",self.name_len);
        println!("{:?}\n",self.name);
    }

}
