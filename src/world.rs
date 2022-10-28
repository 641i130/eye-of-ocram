use std::io::Read;
use std::fs::File;

pub struct World {
    pub file_name: String,
    pub version: i32,   // first byte of file
    pub name: String,  // length is byte before i think...
    pub file_type: i32, // first byte of file
    pub world_timestamp: i32, // first byte of file
    pub w_left: i16,    // World Dimensions
    pub w_right: i32,
    pub w_top: i16,
    pub w_bot: i16,
    pub max_tiles_x: i16,
    pub max_tiles_y: i16,
}
impl World {
    pub fn new(wldfile: &String) -> World {
        World {
            file_name: wldfile.to_string(),
            version: 0, // int 32
            name: "".to_string(),
            file_type: 0, // u8
            world_timestamp: 0, // u8
            w_left: 0,
            w_right: 0,
            w_top: 0,
            w_bot: 0,
            max_tiles_x: 0,
            max_tiles_y: 0,
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
    fn read_i32<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> i32 {
        let mut out:[u8;4] = [0,0,0,0];
        let mut c = 0;
        for _ in 0..4 { // read 4 bytes into i32
            out[c] = World::get_byte(iterator).clone();
            c+=1;
        }
        i32::from_le_bytes(out) as i32
    }
    fn read_i16<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> i16 {
        let mut out:[u8;2] = [0,0];
        let mut c = 0;
        for _ in 0..2 { // read 4 bytes into i32
            out[c] = World::get_byte(iterator).clone();
            c+=1;
        }
        dbg!(out);
        i16::from_le_bytes(out) as i16
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
        let wld = File::open(&self.file_name.to_string()).expect("Cannot read file.");
        let mut iterator = wld.bytes();

        // TODO move to reader with the iterator etc etc
        self.version = World::read_i32(&mut iterator);
        World::skip_bytes(&mut iterator, 4); // Skip 4 bytes! // TODO fix this
        // fileIO.ReadString();
        self.name = World::read_string(&mut iterator); // Read first byte, then read the following
                                                       // bytes as a char array then convert to
                                                       // string
		// int worldID = fileIO.ReadInt32();
        self.file_type = World::read_i32(&mut iterator);
		// int worldTimestamp = (release >= 48) ? fileIO.ReadInt32() : 0;
        self.world_timestamp = World::read_i32(&mut iterator);
		// Main.rightWorld = fileIO.ReadInt32();
        self.w_right = World::read_i32(&mut iterator);
		// Main.bottomWorld = fileIO.ReadInt16();
        self.w_bot = World::read_i16(&mut iterator);
	    // Main.maxTilesY = fileIO.ReadInt16();
        self.max_tiles_y = World::read_i16(&mut iterator);
		// Main.maxTilesX = fileIO.ReadInt16();
        self.max_tiles_x = World::read_i16(&mut iterator);        

        Ok(())
    }
    pub fn pretty_print(self) {
        println!("\nWorld version : {:?}", self.version);
        println!("World Name : {:?}", self.name);
        println!("World dimensions (origin is top left): ");
        println!("\tMax Right:{}", self.w_right);
        println!("\tBottom:{}", self.w_bot);
        println!("Max Tiles X:{}", self.max_tiles_x);
        println!("Max Tiles Y:{}", self.max_tiles_y);
    }
}
