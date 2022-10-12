use std::fs::File;
use std::io::Read;

pub struct WldReader {
    pub wld_path: String,
    pub offset: u32,
    pub size: usize,
    pub buffer: [0;size],
}

impl WldReader {
    pub fn new(wld_path: &String,size: &usize) -> WldReader {
        WldReader {
            wld_path: wld_path.to_string(),
            offset: 0,
            buffer: [0;&usize]
        }
    }
    pub fn ReadInt32(&mut self) {
        // Read a 32 int byte
        return self.buffer[0+offset] 
    }
    pub fn read(&mut self) -> Result<(), Box<dyn std::error::Error>>{
        /// Read into buffer
        
        const BUFFER_SIZE: usize = 256; // TODO make the file size the buffer size probably
        // open target file
        let mut file = File::open(self.wld_path.to_owned())?;
        // we'll use this buffer to get data
        let mut buffer = [0; BUFFER_SIZE];
        // Read file into buffer!!!
        let _ = file.by_ref().take(BUFFER_SIZE.try_into().unwrap()).read(&mut buffer)?;

        // PARSE THE BUFFER NEXT!!! 

        Ok(())
    }
    
}
