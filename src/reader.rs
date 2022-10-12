use std::io::prelude::*;
use std::fs::File;
use std::io::{Read,BufReader};

pub struct WldReader {
    pub wld_path: String,
    pub offset: u32,
    pub input: File,
}

impl WldReader {
    pub fn new(wld_path: &String) -> WldReader {
        // open given world file
        WldReader {
            wld_path: wld_path.to_string(),
            offset: 0,
            input: File::open(wld_path.to_string()).expect("Cannot read file."),
        }
    } 
}
