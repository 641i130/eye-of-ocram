use std::fs::File;
use std::io::Read;

pub struct WldReader {
    pub wld_path: String,
}

impl WldReader {
    pub fn new(wld_path: &String) -> WldReader {
        WldReader {
            wld_path: wld_path.to_string(),
        }
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
