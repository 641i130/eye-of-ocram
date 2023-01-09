/*
#[derive(Debug)]
pub struct Tile {
    active: u8,
    tile_type: u8,
    flags: Flags,
    liquid: u8,
    lava: u8,
    wall: u8,
    wall_frame_x: u16,
    wall_frame_y: u8,
    frame_number: u8,
    frame_x: i16,
    frame_y: i16,
    wall_frame_number: i32,
    checking_liquid: i32,
    skip_liquid: i32,
    wire: i32,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Flags {
    WallframeMask = 0x3,
    Nearby = 0x4,
    Visited = 0x8,
    Wire = 0x10,
    Selected = 0x20,
    Lava = 0x20,
    CheckingLiquid = 0x40,
    SkipLiquid = 0x80,
    HighlightMask = 0x24,
}

pub impl Tile {
    fn new() -> Self {
        Tile {
            active: 0,
            tile_type: 0,
            flags: Flags::WallframeMask,
            liquid: 0,
            lava: 0,
            wall: 0,
            wall_frame_x: 0,
            wall_frame_y: 0,
            frame_number: 0,
            frame_x: 0,
            frame_y: 0,
            wall_frame_number: 0,
            checking_liquid: 0,
            skip_liquid: 0,
            wire: 0,
        }
    }

    fn clear(&mut self) {
        self.active = 0;
        // Zero out flags ~ invertt (make it all 111111)
        self.flags = !(Flags::WallframeMask | Flags::Nearby | Flags::Visited | Flags::Wire | Flags::Selected | Flags::CheckingLiquid | Flags::SkipLiquid);
        self.tile_type = 0;
        self.wall = 0;
        self.wall_frame_x = 0;
        self.wall_frame_y = 0;
        self.liquid = 0;
        self.lava = 0;
        self.frame_number = 0;
    }

    fn is_the_same_as_excluding_visibility(&self, comp_tile: &Tile) -> bool {
        if self.active != comp_tile.active {
            return false;
        }
        if self.active != 0 {
            if self.tile_type != comp_tile.tile_type {
                return false;
            }
            if Main::tile_frame_important[self.tile_type as usize] {
                if self.frame_x != comp_tile.frame_x {
                    return false;
                }
               
*/
