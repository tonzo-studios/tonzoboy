mod cpu;
mod register;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.step();
    // cpu.run(&rom_path) or something
}
