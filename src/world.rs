pub struct World {
    pub version: u8, // first byte of file
    name_len: u8, //,,
    pub name: String, // length is byte before i think...
}
