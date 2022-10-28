use std::io::Read;
use std::fs::File;

enum Flags {
    wallframe_mask = 0x3,
    nearby = 0x4,
    visited = 0x8,
    wire = 0x10,
    selected = 0x20,
    lava = 0x20,
    checking_liquid = 0x40,
    skip_liquid = 0x80,
    highlight_mask = 0x24,
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
pub impl Tile {
    fn get_wallframe_num(&self) -> i32 {
        // This casts the flags bytes into i32 i think
        // (int)(flags & Flags.WALLFRAME_MASK);
        (self.flags & Flags.wallframe_mask) as i32
    }
    fn set_wallframe_num(&mut self) {
        self.flags = (Flags.WALLFRAME_MASK | Flags.)
    }
}

#[derive(Debug)]
pub struct World {
    pub file_name: String,
    pub version: i32,   // first byte of file
    pub name: String,  // length is byte before i think...
    pub file_type: i32, // first byte of file
    pub world_timestamp: i32, // first byte of file
    pub w_left: i16,    // World Dimensions
    pub w_right: i32, // ReadInt32
    pub w_top: i16,
    pub w_bot: i16,
    pub max_tiles_x: i16,
    pub max_tiles_y: i16,
    pub spawn_tile_x: i16,
    pub spawn_tile_y: i16,
    pub world_surface: i16,
    pub rock_layer: i16, // ReadInt16
    pub world_time: f32, // ReadSingle
    pub day_time: bool, // 0 is false && else is true
    pub moon_phase: u8, // ReadByte
    pub blood_moon: bool,
    pub dungeonx: i16,
    pub dungeony: i16,
    pub boss_1_down: bool,
    pub boss_2_down: bool,
    pub boss_3_down: bool,
    pub saved_goblin: bool,
    pub saved_wizard: bool,
    pub saved_mech: bool,
    pub goblins_down: bool,
    pub clown_down: bool,
    pub frost_down: bool,
    pub shadow_orb_smashed: bool,
    pub spawn_meteor: bool,
    pub altar_count: u8,
    pub hard_mode: bool,
    pub invasion_delay: u8,
    pub invasion_size: i16,
    pub invasion_type: u8,
    pub invasionx: f32,
    // 1887


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
            spawn_tile_x: 0,
            spawn_tile_y: 0,
            world_surface: 0,
            rock_layer: 0,
            world_time: 0.0, // ReadSingle
            day_time: false, // 0 is false && else is true
            moon_phase: 0, // ReadByte
            blood_moon: false,
            dungeonx: 0,
            dungeony: 0,
            boss_1_down: false,
            boss_2_down: false,
            boss_3_down: false,
            saved_goblin: false,
            saved_wizard: false,
            saved_mech: false,
            goblins_down: false,
            clown_down: false,
            frost_down: false,
            shadow_orb_smashed: false,
            spawn_meteor: false,
            altar_count: 0,
            hard_mode: false,
            invasion_delay: 0,
            invasion_size: 0,
            invasion_type: 0,
            invasionx: 0.0,
        }
    }
    fn convert_using_from_iter<const N:usize,T,E> (r : [Result<T, E>; N]) -> Result<[T; N], E>  where T : std::fmt::Debug {
        // Helper function for file::iterator into i32 (from byte array)
        let result : Vec<_> = Result::from_iter (r.into_iter ())?;
        Ok (result.try_into ().unwrap ())
    }
    fn get_byte<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> u8 {
        iterator.next().expect("No more bytes").unwrap()
    }
    fn read_bool<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> Result<bool,Box<dyn std::error::Error>> {
        if iterator.next().expect("No more bytes").unwrap() != 0 {
            return Ok(true);
        }
        return Ok(false);
    }
    fn skip_bytes<R: std::io::Read>(iterator: &mut std::io::Bytes<R>, count: usize) {
        for _ in 0..count {
            let _ = iterator.next().expect("No more bytes!").unwrap();
        }
    }
    fn read_i32<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> Result<i32,Box<dyn std::error::Error>> {
        Ok(i32::from_le_bytes(Self::convert_using_from_iter (iterator.next_chunk::<4>().map_err (|_e| "could not read 4 bytes")?)?))
    }
    fn read_single<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> Result<f32,Box<dyn std::error::Error>> {
        Ok(f32::from_le_bytes(Self::convert_using_from_iter (iterator.next_chunk::<4>().map_err (|_e| "could not read 4 bytes")?)?) as f32)
    }
    fn read_i16<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> Result<i16,Box<dyn std::error::Error>> {
        Ok(i16::from_le_bytes(Self::convert_using_from_iter (iterator.next_chunk::<2>().map_err (|_e| "could not read 4 bytes")?)?))
    }
    fn read_string<R: std::io::Read>(iterator: &mut std::io::Bytes<R>) -> Result<String,Box<dyn std::error::Error>> {
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
        Ok(out.iter().cloned().collect::<String>())
    }
    pub fn read_wld(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let wld = File::open(&self.file_name.to_string()).expect("Cannot read file.");
        let mut iterator = wld.bytes();

        // TODO move to reader with the iterator etc etc
        self.version = World::read_i32(&mut iterator)?;
        World::skip_bytes(&mut iterator, 4); // Skip 4 bytes! // TODO fix this
        // fileIO.ReadString();
        self.name = World::read_string(&mut iterator)?; // Read first byte, then read the following
                                                       // bytes as a char array then convert to
                                                       // string
		// int worldID = fileIO.ReadInt32();
        self.file_type = World::read_i32(&mut iterator)?;
		// int worldTimestamp = (release >= 48) ? fileIO.ReadInt32() : 0;
        if self.version >= 48 {
            self.world_timestamp = World::read_i32(&mut iterator)?;
        }
		// Main.rightWorld = fileIO.ReadInt32();
        self.w_right = World::read_i32(&mut iterator)?;
		// Main.bottomWorld = fileIO.ReadInt16();
        self.w_bot = World::read_i16(&mut iterator)?;
	    // Main.maxTilesY = fileIO.ReadInt16();
        self.max_tiles_y = World::read_i16(&mut iterator)?;
		// Main.maxTilesX = fileIO.ReadInt16();
        self.max_tiles_x = World::read_i16(&mut iterator)?;        
        // 1851
        
        // Main.spawnTileX = fileIO.ReadInt16();
        self.spawn_tile_x = World::read_i16(&mut iterator)?;
        // Main.spawnTileY = fileIO.ReadInt16();
        self.spawn_tile_y = World::read_i16(&mut iterator)?;
        // Main.worldSurface = fileIO.ReadInt16();
        self.world_surface = World::read_i16(&mut iterator)?;
        // Main.worldSurfacePixels = Main.worldSurface << 4;
        // Main.rockLayer = fileIO.ReadInt16();
        self.rock_layer = World::read_i16(&mut iterator)?;
        // Main.rockLayerPixels = Main.rockLayer << 4;
        // 1862
        self.world_time = World::read_single(&mut iterator)?;
        self.day_time = World::read_bool(&mut iterator)?;
        self.moon_phase = World::get_byte(&mut iterator);
        self.blood_moon = World::read_bool(&mut iterator)?;
        self.dungeonx = World::read_i16(&mut iterator)?;
        self.dungeony = World::read_i16(&mut iterator)?;
        self.boss_1_down = World::read_bool(&mut iterator)?;
        self.boss_2_down = World::read_bool(&mut iterator)?;
        self.boss_3_down = World::read_bool(&mut iterator)?;
        self.saved_goblin = World::read_bool(&mut iterator)?;
        self.saved_wizard = World::read_bool(&mut iterator)?;
        self.saved_mech = World::read_bool(&mut iterator)?;
        self.goblins_down = World::read_bool(&mut iterator)?;
        self.clown_down = World::read_bool(&mut iterator)?;
        self.frost_down = World::read_bool(&mut iterator)?;
        self.shadow_orb_smashed = World::read_bool(&mut iterator)?;
        self.spawn_meteor = World::read_bool(&mut iterator)?;
        self.altar_count = World::get_byte(&mut iterator);
        self.hard_mode = World::read_bool(&mut iterator)?;
        self.invasion_delay = World::get_byte(&mut iterator);
        self.invasion_size = World::read_i16(&mut iterator)?;
        self.invasion_type = World::get_byte(&mut iterator);
        self.invasionx = World::read_single(&mut iterator)?;

        Ok(())
    }
    pub fn pretty_print(self) {
        dbg!(self);
    }
}
