use cpu::{Cpu, CpuRegister};

pub fn subtract(cpu: &mut Cpu, val: u8) {
    let lhs: u8 = cpu.get(CpuRegister::A);

    let result: u8 = lhs.wrapping_sub(val);

    let z = result == 0;
    let h = false;
    let c = false;

    cpu.set_flags(z, true, h, c);

    // Flag N = 1
    // Flags Z, H, C
}

pub fn increment(cpu: &mut Cpu, reg: CpuRegister) {
    let val: u8 = cpu.get(reg);
    let newval = val.wrapping_add(1);
    cpu.set(reg, newval);

    let z = newval == 0;
    let h = false;
    let c: bool = cpu.c_flag() == 1;

    cpu.set_flags(z, false, h, c);
}

pub fn decrement(cpu: &mut Cpu, reg: CpuRegister) {
    let val: u8 = cpu.get(reg);
    let newval = val.wrapping_sub(1);
    cpu.set(reg, newval);

    let z = newval == 0;
    let h = false;
    let c: bool = cpu.c_flag() == 1;

    cpu.set_flags(z, true, h, c);
}

pub fn complement(cpu: &mut Cpu) {
    panic!("complement not implemented");
}
