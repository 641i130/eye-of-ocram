use std::fs::File;
use std::io::Read;

pub struct World {
    pub file_name: String,
    pub version: u8, // first byte of file
    pub name_len: u8, // the byte at someplace
    pub name: String, // length is byte before i think...
}
impl World {
    pub fn new(wldfile:&String) -> World { 
        World {
            file_name: wldfile.to_string(),
            version: 0,
            name_len: 0,
            name: "".to_string(),
        }
    }
    pub fn read_wld(&mut self) -> Result<(), std::io::Error> {
        const BUFFER_SIZE: usize = 256;

        // open target file
        let mut file = File::open(self.file_name.to_owned())?;

        // we'll use this buffer to get data
        let mut buffer = [0; BUFFER_SIZE];

        // Read file into buffer!!!
        let _ = file.by_ref().take(BUFFER_SIZE.try_into().unwrap()).read(&mut buffer)?;
        // print out every value
        self.version = buffer[0];
        println!("World version : {:?}",self.version);
        if self.version == 69 {
            self.name_len = buffer[8];
            println!("World name is {:?} bits / 4 characters long.",self.name_len);
            self.name = std::str::from_utf8(&buffer[9 as usize..self.name_len as usize]).unwrap().to_string();
            println!("{:?}",self.name);
            print!("\n");
            
        }
        for v in &buffer {
            print!("{:02X} ",v);
        }
        Ok(())
    }

}
