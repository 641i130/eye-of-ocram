#[feature(iter_next_chunk)]
mod world;
use world::World;

use std::env;

fn help() {
    eprintln!("Usage:\n -w [file location]");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut i: usize = 0;
    let mut world_file: String = String::new();
    for arg in &args {
        // for each argument
        match arg.as_ref() {
            // do stuff with argument inputs (if the argument equals the strings below, do ...)
            "-w" => {
                world_file = args[i + 1].parse::<String>().unwrap();
                println!("World file to parse : {:?}", world_file);
                i += 1;
            }
            "-h" => {
                help();
            }
            &_ => i += 1,
        }
    }
    if &world_file != "" {
        // INIT world struct
        let mut test = World::new(&world_file);
        match test.read_wld() {
            Ok(_) => {
                println!("Read successfully!");
                test.pretty_print();
            }
            Err(_) => help(),
        }
    }
}
