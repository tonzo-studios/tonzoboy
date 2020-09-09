mod cpu;
mod memory;
mod register;

use clap::{App, Arg};
use std::path::Path;

use cpu::Cpu;

fn main() {
    let matches = App::new("tonzoboy")
        .arg(
            Arg::with_name("file")
                .short("f")
                .required(true)
                .index(1)
                .help("Path of the ROM file to load"),
        )
        .get_matches();
    let rom_path = Path::new(matches.value_of("file").unwrap());
    let mut cpu = Cpu::new(&rom_path);
    cpu.run()
}
