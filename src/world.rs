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
    fn get_byte<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> u8 {
        iterator.next().expect("No more bytes").unwrap()
    }
    fn skip_bytes<R: std::io::Read>(iterator: &mut std::io::Bytes<R>,count: u8) {
        for _ in 0..count {
            let _ = iterator.next().expect("No more bytes!").unwrap();
        }
    }
    fn read_string<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> String {
        let mut out = vec![]; // Build string vector
        for _ in 0..World::get_byte(iterator) { // Get string length 
            out.push(World::get_byte(iterator) as char);
        }
        out.iter().cloned().collect::<String>()
    }
    pub fn read_wld(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut wldp = WldReader::new(&self.file_name);
        let mut iterator = wldp.input.bytes();
        self.version = World::get_byte(&mut iterator);
        World::skip_bytes(&mut iterator,7); // Skip 8 bytes!
        self.name = World::read_string(&mut iterator);
        Ok(())
    }
    pub fn pretty_print(self) {
        println!("\nWorld version : {:?}",self.version);
        println!("World Name : {:?}",self.name);
        println!("{:?}\n",self.name);
    }

}
