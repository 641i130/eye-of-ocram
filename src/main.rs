use std::env;
use std::fs::File;
use std::io::Read;
fn help() {
    eprintln!("Usage:\n -w [file location]");
}

fn read_wld(file_name: &str) -> Result<(), std::io::Error> {
    const BUFFER_SIZE: usize = 256;

    // open target file
    let mut file = File::open(&file_name)?;

    // we'll use this buffer to get data
    let mut buffer = [0; BUFFER_SIZE];

    // reader WLD version
    let _ = file.by_ref().take(BUFFER_SIZE.try_into().unwrap()).read(&mut buffer)?;
    for v in &buffer {
        print!("{:02X}",v);
    }
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut i:usize = 0;
    let mut world_file:String = String::new();
    for arg in &args { // for each argument
        match arg.as_ref() { // do stuff with argument inputs (if the argument equals the strings below, do ...)
            "-w" => {
                world_file = args[i+1].parse::<String>().unwrap();
                println!("World file to parse : {:?}", world_file);
                i+=1;
            },
            "-h" => {
                help();
                std::process::exit(1);
            }
            &_ => i+=1,
        }
    }
    if &world_file != "" {
        read_wld(&world_file);
    }
}
