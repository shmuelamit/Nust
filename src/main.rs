mod cpu;
use cpu::*;
fn main() {
	let cpu = &mut Cpu::default();
	cpu.status.insert(CpuFlags::all());
	println!("{}", cpu);
}
