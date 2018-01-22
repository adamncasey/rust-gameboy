use cpu::{Cpu, Cpu16Register, CpuRegister};

pub fn subtract(cpu: &mut Cpu, val: u8) {
    let lhs: u8 = cpu.get(CpuRegister::A);

    let result: u8 = lhs.wrapping_sub(val);

    cpu.set(CpuRegister::A, result);

    let z = result == 0;
    let h = false; // TODO
    let c = false; // TODO

    cpu.set_flags(z, true, h, c);
}

pub fn add(cpu: &mut Cpu, val: u8) {
    let lhs: u8 = cpu.get(CpuRegister::A);

    let result: u8 = lhs.wrapping_add(val);

    cpu.set(CpuRegister::A, result);

    let z = result == 0;
    let h = false; // TODO
    let c = false; // TODO

    cpu.set_flags(z, true, h, c);
}

pub fn add16(cpu: &mut Cpu, reg: Cpu16Register, val: u16) {
    let lhs: u16 = cpu.get16(reg);

    let result: u16 = lhs.wrapping_add(val);

    cpu.set16(reg, result);

    let z = cpu.z_flag();
    // TODO: H C
    cpu.set_flags(z, false, false, false);
}

pub fn increment(cpu: &mut Cpu, val: u8) -> u8 {
    let newval = val.wrapping_add(1);

    let z = newval == 0;
    let h = false; // TODO
    let c: bool = cpu.c_flag() == 1;

    cpu.set_flags(z, false, h, c);

    return newval;
}

pub fn increment16(cpu: &mut Cpu, reg: Cpu16Register) {
    let val: u16 = cpu.get16(reg);
    let newval = val.wrapping_add(1);
    cpu.set16(reg, newval);

    let z = newval == 0;
    let h = false; // TODO
    let c: bool = cpu.c_flag() == 1;

    cpu.set_flags(z, false, h, c);
}

pub fn decrement(cpu: &mut Cpu, val: u8) -> u8 {
    let newval = val.wrapping_sub(1);

    let z = newval == 0;
    let h = false; // TODO
    let c: bool = cpu.c_flag() == 1;

    cpu.set_flags(z, true, h, c);

    return newval;
}

pub fn decrement16(cpu: &mut Cpu, reg: Cpu16Register) {
    let val: u16 = cpu.get16(reg);
    let newval = val.wrapping_sub(1);
    cpu.set16(reg, newval);

    let z = newval == 0;
    let h = false;
    let c: bool = cpu.c_flag() == 1;

    cpu.set_flags(z, true, h, c);
}

pub fn complement(cpu: &mut Cpu) {
    let z = cpu.z_flag();
    let c = cpu.c_flag() == 1;
    cpu.set_flags(z, true, true, c);

    cpu.a = !cpu.a;
}

pub fn xor(cpu: &mut Cpu, val: u8) {
    let result: u8 = cpu.get(CpuRegister::A) ^ val;

    cpu.a = result;

    cpu.set_flags(result == 0, false, false, false);
}

pub fn or(cpu: &mut Cpu, val: u8) {
    let result: u8 = cpu.get(CpuRegister::A) | val;

    cpu.a = result;

    cpu.set_flags(result == 0, false, false, false);
}

pub fn and(cpu: &mut Cpu, val: u8) {
    let result: u8 = cpu.get(CpuRegister::A) & val;

    cpu.a = result;

    cpu.set_flags(result == 0, false, false, false);
}

pub fn compare(cpu: &mut Cpu, val: u8) {
    let a: u8 = cpu.get(CpuRegister::A);

    let z = a == val;
    let n = true;
    let h = false; // TODO
    let c = a < val;

    cpu.set_flags(z, n, h, c);
}

pub fn swap_nibble(cpu: &mut Cpu, val: u8) -> u8 {
    let res = ((val & 0xF0) >> 4) | ((val & 0x0F) << 4);

    cpu.set_flags(val == 0, false, false, false);

    return res;
}

pub fn bit(cpu: &mut Cpu, val: u8, n: u8) {
    let res = val & (0b1 << n);

    let c = cpu.c_flag() != 0;
    cpu.set_flags(res == 0, false, true, c);
}

pub fn set(val: u8, n: u8) -> u8 {
    let mask = 0b1 << n;

    return val | mask;
}

pub fn reset(val: u8, n: u8) -> u8 {
    let mask = 0b1 << n;

    return !((!val) | mask);
}

pub fn sla(cpu: &mut Cpu, val: u8) -> u8 {
    let res = val << 1;
    let msb = (val & 0b10000000) != 0;

    cpu.set_flags(res == 0, false, false, msb);

    res
}