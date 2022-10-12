use std::io::prelude::*;
use std::fs::File;
use std::io::{Read,BufReader};

pub struct WldReader {
    pub wld_path: String,
    pub offset: u32,
    pub buffer: BufReader<File>,
}

impl WldReader {
    pub fn new(wld_path: &String) -> WldReader {
        // open given world file
        let file = File::open(wld_path.to_string()).expect("Cannot read file.");
        let mut buf = BufReader::new(file);
        WldReader {
            wld_path: wld_path.to_string(),
            offset: 0,
            buffer: buf,
        }
    }
    pub fn ReadInt32(&mut self) {
        // Read a 32 int byte
//        return self.buffer[0+self.offset] 
    }    
}
