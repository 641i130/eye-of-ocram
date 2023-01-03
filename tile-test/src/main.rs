#[derive(Debug)]
pub enum Flags {
    WallframeMask = 0x3,
    Nearby = 0x4,
    Visited = 0x8,
    Wire = 0x10,
    Selected = 0x20,
    Lava = 0x21,
    CheckingLiquid = 0x40,
    SkipLiquid = 0x80,
    HighlightMask = 0x24,
}

#[derive(Debug)]
pub struct Tile {
    pub active: u8, // all byte types should be this type in rust
    pub t_type: u8,
    pub flags: Flags,
    pub liquid: u8,
    pub lava: u8,
    pub wall: u8,
    pub wallframe_x: u16, // ushort
    pub wallframe_y: u16, // ushort
    pub frame_num: u8,
    pub frame_x: u8,
    pub frame_y: u8,
}

fn main() {
    let tiles:Tile = Tile;
    dbg!(&tiles);
}
