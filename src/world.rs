use crate::reader::WldReader;
use std::io::Read;

pub struct World {
    pub file_name: String,
    pub version: u8,   // first byte of file
    pub name: String,  // length is byte before i think...
    pub file_type: u8, // first byte of file
    pub w_left: u8,    // World Dimensions
    pub w_right: u8,
    pub w_top: u8,
    pub w_bot: u8,
    pub max_tiles_x: u8,
    pub max_tiles_y: u8,
    pub moon_type: u8,
}
impl World {
    pub fn new(wldfile: &String) -> World {
        World {
            file_name: wldfile.to_string(),
            version: 0, // int 32
            name: "".to_string(),
            file_type: 0, // u8
            w_left: 0,
            w_right: 0,
            w_top: 0,
            w_bot: 0,
            max_tiles_x: 0,
            max_tiles_y: 0,
            moon_type: 0,
        }
    }
    fn get_byte<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> u8 {
        iterator.next().expect("No more bytes").unwrap()
    }
    fn skip_bytes<R: std::io::Read>(iterator: &mut std::io::Bytes<R>, count: u8) {
        for _ in 0..count {
            let _ = iterator.next().expect("No more bytes!").unwrap();
        }
    }
    fn read_string<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> String {
        // readString(length) {
        // if (length === undefined) { //7 bit encoded int32
        //     length = 0;
        //     let shift = 0, byte;
        //     do {
        //         byte = this.readUInt8();
        //         length |= (byte & 127) << shift;
        //         shift += 7;
        // } while (byte & 128);
        // return new TextDecoder().decode(this.readBytes(length));
        // }
        let mut out = vec![]; // Build string vector
        for _ in 0..World::get_byte(iterator) {
            // Get string length
            out.push(World::get_byte(iterator) as char);
        }
        out.iter().cloned().collect::<String>()
    }
    pub fn read_wld(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let wldp = WldReader::new(&self.file_name);
        let mut iterator = wldp.input.bytes();

        // TODO move to reader with the iterator etc etc
        self.version = World::get_byte(&mut iterator);
        World::skip_bytes(&mut iterator, 7); // Skip 7 bytes!
        self.name = World::read_string(&mut iterator); // Read first byte, then read the following
                                                       // bytes as a char array then convert to
                                                       // string
        
        // USED FOR MAP STUFF ???
        self.file_type = World::get_byte(&mut iterator);

        self.w_left = World::get_byte(&mut iterator);
        self.w_right = World::get_byte(&mut iterator);
        self.w_top = World::get_byte(&mut iterator);
        self.w_bot = World::get_byte(&mut iterator);

        self.max_tiles_y = World::get_byte(&mut iterator);
        self.max_tiles_x = World::get_byte(&mut iterator);
        
        self.moon_type = World::get_byte(&mut iterator); // maybe its actually creation time
        Ok(())
    }
    pub fn pretty_print(self) {
        println!("\nWorld version : {:?}", self.version);
        println!("World Name : {:?}", self.name);
        println!("World dimensions (origin is top left): ");
        println!("\tL:{} R:{}", self.w_left as i32, self.w_right as i32);
        println!("\tT:{} B:{}", self.w_top as i32, self.w_bot as i32);
        println!("Max Tiles X:{}", self.max_tiles_x as i32);
        println!("Max Tiles Y:{}", self.max_tiles_y as i32);
        println!("Moon type:{}",self.moon_type); // My guess is there are these options:
                                                      // 10011111
                                                      // 01011111
                                                      // 00111111
    }
}
