use cpu::{Cpu, CpuRegister, Cpu16Register};

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

pub fn add16(cpu: &mut Cpu, val: u16) {
    let lhs: u16 = cpu.get16(Cpu16Register::HL);

    let result: u16 = lhs.wrapping_add(val);

    cpu.set16(Cpu16Register::HL, result);

    let z = cpu.z_flag();
    // TODO: H C
    cpu.set_flags(z, false, false, false);
}

pub fn increment(cpu: &mut Cpu, reg: CpuRegister) {
    let val: u8 = cpu.get(reg);
    let newval = val.wrapping_add(1);
    cpu.set(reg, newval);

    let z = newval == 0;
    let h = false; // TODO
    let c: bool = cpu.c_flag() == 1;

    cpu.set_flags(z, false, h, c);
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

pub fn decrement(cpu: &mut Cpu, reg: CpuRegister) {
    let val: u8 = cpu.get(reg);
    let newval = val.wrapping_sub(1);
    cpu.set(reg, newval);

    let z = newval == 0;
    let h = false; // TODO
    let c: bool = cpu.c_flag() == 1;

    cpu.set_flags(z, true, h, c);
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

pub fn compare(cpu: &mut Cpu, val: u8)
{
    let a: u8 = cpu.get(CpuRegister::A);

    let z = a == val;
    let n = true;
    let h = false; // TODO
    let c = a < val;

    cpu.set_flags(z, n, h, c);
}