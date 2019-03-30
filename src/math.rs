use crate::cpu::{Cpu, Cpu16Register, CpuRegister};

pub fn subtract(cpu: &mut Cpu, val: u8) {
    let lhs: u8 = cpu.get(CpuRegister::A);

    let result: u8 = lhs.wrapping_sub(val);

    cpu.set(CpuRegister::A, result);

    let z = result == 0;
    let h = ((lhs & 0xf) as i8 - (val & 0xf) as i8) < 0;
    let c = lhs > val;

    cpu.set_flags(z, true, h, c);
}

pub fn add(cpu: &mut Cpu, val: u8) {
    let lhs: u8 = cpu.get(CpuRegister::A);

    let result: u8 = lhs.wrapping_add(val);

    cpu.set(CpuRegister::A, result);

    let z = result == 0;
    let h = ((lhs & 0xf).wrapping_add(val & 0xf) & 0x10) != 0;
    let c = result <= lhs && (val != 0);

    cpu.set_flags(z, false, h, c);
}

pub fn add16(cpu: &mut Cpu, reg: Cpu16Register, val: u16) {
    let lhs: u16 = cpu.get16(reg);

    let result: u16 = lhs.wrapping_add(val);

    cpu.set16(reg, result);

    let z = cpu.z_flag();
    let h = ((lhs & 0xf00).wrapping_add(val & 0xf00) & 0x10) != 0;
    let c = result <= lhs && (val != 0);
    cpu.set_flags(z, false, h, c);
}

pub fn increment(cpu: &mut Cpu, val: u8) -> u8 {
    let newval = val.wrapping_add(1);

    let z = newval == 0;
    let h = (val & 0xf) == 0xf;
    let c: bool = cpu.c_flag();

    cpu.set_flags(z, false, h, c);

    newval
}

pub fn increment16(cpu: &mut Cpu, reg: Cpu16Register) {
    let val: u16 = cpu.get16(reg);
    let newval = val.wrapping_add(1);
    cpu.set16(reg, newval);

    let z = newval == 0;
    let h = (val & 0xf00) == 0xf00;
    let c: bool = cpu.c_flag();

    cpu.set_flags(z, false, h, c);
}

pub fn decrement(cpu: &mut Cpu, val: u8) -> u8 {
    let newval = val.wrapping_sub(1);

    let z = newval == 0;
    let h = (newval & 0xf) == 0xf;
    let c: bool = cpu.c_flag();

    cpu.set_flags(z, true, h, c);

    newval
}

pub fn decrement16(cpu: &mut Cpu, reg: Cpu16Register) {
    let val: u16 = cpu.get16(reg);
    let newval = val.wrapping_sub(1);
    cpu.set16(reg, newval);

    let z = newval == 0;
    let h = (newval & 0xf00) == 0xf00;
    let c: bool = cpu.c_flag();

    cpu.set_flags(z, true, h, c);
}

pub fn complement(cpu: &mut Cpu) {
    let z = cpu.z_flag();
    let c = cpu.c_flag();
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
    let h = ((a & 0xf) as i8 - (val & 0xf) as i8) < 0;
    let c = a < val;

    cpu.set_flags(z, n, h, c);
}

pub fn swap_nibble(cpu: &mut Cpu, val: u8) -> u8 {
    let res = ((val & 0xF0) >> 4) | ((val & 0x0F) << 4);

    cpu.set_flags(val == 0, false, false, false);

    res
}

pub fn bit(cpu: &mut Cpu, val: u8, n: u8) {
    let res = val & (0b1 << n);

    let c = cpu.c_flag();
    cpu.set_flags(res == 0, false, true, c);
}

pub fn set(val: u8, n: u8) -> u8 {
    let mask = 0b1 << n;

    val | mask
}

pub fn reset(val: u8, n: u8) -> u8 {
    let mask = 0b1 << n;

    !((!val) | mask)
}

pub fn sla(cpu: &mut Cpu, val: u8) -> u8 {
    let res = val << 1;
    let msb = (val & 0b1000_0000) != 0;

    cpu.set_flags(res == 0, false, false, msb);

    res
}

pub fn srl(cpu: &mut Cpu, val: u8) -> u8 {
    let res = val >> 1;
    let lsb = (val & 0b1) != 0;

    cpu.set_flags(res == 0, false, false, lsb);

    res
}

pub fn sra(cpu: &mut Cpu, val: u8) -> u8 {
    let msb = val & 0b1000_0000;
    let res = (val >> 1) | msb;
    let lsb = (val & 0b1) != 0;

    cpu.set_flags(res == 0, false, false, lsb);

    res
}

pub fn rr(cpu: &mut Cpu, val: u8) -> u8 {
    let old_c = (cpu.c_flag() as u8) << 7;
    let c: bool = (val & 0b1) != 0;

    let rotated = val.rotate_right(1);
    let res = rotated | old_c;

    cpu.set_flags(res == 0, false, false, c);

    res
}

pub fn rl(cpu: &mut Cpu, val: u8) -> u8 {
    let old_c = cpu.c_flag() as u8;
    let c: bool = (val & 0b1000_0000) != 0;

    let rotated = val.rotate_left(1);
    let res = rotated | old_c;

    cpu.set_flags(res == 0, false, false, c);

    res
}
pub fn rrc(cpu: &mut Cpu, val: u8) -> u8 {
    let c: bool = (val & 0b1) != 0;
    let res = val.rotate_right(1);
    cpu.set_flags(res == 0, false, false, c);

    res
}
pub fn rlc(cpu: &mut Cpu, val: u8) -> u8 {
    let c: bool = (val & 0b1000_0000) != 0;
    let res = val.rotate_left(1);
    cpu.set_flags(res == 0, false, false, c);

    res
}

pub fn complement_carry(cpu: &mut Cpu) {
    let z: bool = cpu.z_flag();

    cpu.set_flags(z, false, false, !cpu.c_flag());
}

pub fn daa(cpu: &mut Cpu) {
    let h_flag = cpu.h_flag();
    let c_flag = cpu.c_flag();
    let n_flag = cpu.n_flag();

    let mut adjust: u8 = 0;
    let mut carry = false;

    if h_flag || (!n_flag && ((cpu.a & 0x0F) > 0x09)) {
        adjust = 0x06;
    }

    if c_flag || (!n_flag && ((cpu.a & 0xF0) > 0x90)) {
        adjust |= 0x60;
        carry = true;
    }

    if n_flag {
        cpu.a.wrapping_sub(adjust);
    } else {
        cpu.a.wrapping_add(adjust);
    }

    cpu.set_flags(cpu.a == 0, n_flag, false, carry);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        assert_eq!(0x01, set(0, 0));
        assert_eq!(0x02, set(0, 1));
        assert_eq!(0x04, set(0, 2));
        assert_eq!(0x08, set(0, 3));
        assert_eq!(0x10, set(0, 4));
        assert_eq!(0x20, set(0, 5));
        assert_eq!(0x40, set(0, 6));
        assert_eq!(0x80, set(0, 7));

        assert_eq!(0x01, set(1, 0));
    }

    #[test]
    fn test_cast() {
        assert_eq!(255, i16::from(0xFF as u8));
    }

    #[test]
    fn test_add_half_full_carry_zero() {
        let mut cpu = Cpu::new();

        cpu.a = 0xFF;

        add(&mut cpu, 0x01);

        assert_eq!(cpu.a, 0x00);

        let expected_flags = 0b1011_0000;
        assert_eq!(cpu.f, expected_flags);
    }
    #[test]
    fn test_add_half_carry() {
        let mut cpu = Cpu::new();

        cpu.a = 0x0F;

        add(&mut cpu, 0x01);

        assert_eq!(cpu.a, 0x10);

        let expected_flags = 0b0010_0000;
        assert_eq!(cpu.f, expected_flags);
    }
}
