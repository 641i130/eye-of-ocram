use crate::reader::WldReader;
use std::io::Read;

pub struct World {
    pub file_name: String,
    pub version: u8, // first byte of file
    pub name: String, // length is byte before i think...
    pub file_type: u8, // first byte of file
}
impl World {
    pub fn new(wldfile:&String) -> World { 
        World {
            file_name: wldfile.to_string(),
            version: 0, // int 32 
            name: "".to_string(),
            file_type: 0 // u8
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
        /// readString(length) {
        /// if (length === undefined) { //7 bit encoded int32
        ///     length = 0;
        ///     let shift = 0, byte;
        ///     do {
        ///         byte = this.readUInt8();
        ///         length |= (byte & 127) << shift;
        ///         shift += 7;
        /// } while (byte & 128);
        /// return new TextDecoder().decode(this.readBytes(length));
        /// }
        let mut out = vec![]; // Build string vector
        for _ in 0..World::get_byte(iterator) { // Get string length 
            out.push(World::get_byte(iterator) as char);
        }
        out.iter().cloned().collect::<String>()
    }
    pub fn read_wld(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut wldp = WldReader::new(&self.file_name);
        let mut iterator = wldp.input.bytes();

        // TODO move to reader with the iterator etc etc
        self.version = World::get_byte(&mut iterator); 
        World::skip_bytes(&mut iterator,7); // Skip 7 bytes!
        self.name = World::read_string(&mut iterator); // Read first byte, then read the following
                                                       // bytes as a char array then convert to
                                                       // string
        self.file_type = World::get_byte(&mut iterator); 
        Ok(())
    }
    pub fn pretty_print(self) {
        println!("\nWorld version : {:?}",self.version);
        println!("World Name : {:?}",self.name);
    }

}
