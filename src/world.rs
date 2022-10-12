use crate::reader::WldReader;
use std::io::Read;

pub struct World {
    pub file_name: String,
    pub version: u8, // first byte of file
    pub name: String, // length is byte before i think...
}
impl World {
    pub fn new(wldfile:&String) -> World { 
        World {
            file_name: wldfile.to_string(),
            version: 0, // int 32 
            name: "".to_string(),
        }
    }
    fn get_byte<R: std::io::Read>(mut iterator: std::io::Bytes<R>) -> u8 {
        iterator.next().expect("").unwrap()
    }
    pub fn read_wld(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut wldp = WldReader::new(&self.file_name);
        let mut iterator = wldp.input.bytes();
        println!("{:?}",World::get_byte(iterator)); 
        Ok(())
    }
    pub fn pretty_print(self) {
        println!("\nWorld version : {:?}",self.version);
        println!("World Name : {:?}",self.name);
        println!("{:?}\n",self.name);
    }

}
