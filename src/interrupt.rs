use crate::memory::Memory;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Interrupt {
    VBlank,
    LcdStat,
    /* Serial, */
    Timer,
    Joypad,
}

const INT_VBLANK: u8 = 1;
const INT_LCDSTAT: u8 = 2;
const INT_TIMER: u8 = 4;
const INT_JOYPAD: u8 = 16;

pub fn set_interrupt(int: Interrupt, mem: &mut Memory) {
    let current = mem.get(0xFF0F);

    let new = match int {
        Interrupt::VBlank => current | INT_VBLANK,
        Interrupt::LcdStat => current | INT_LCDSTAT,
        Interrupt::Timer => current | INT_TIMER,
        Interrupt::Joypad => current | INT_JOYPAD,
    };

    mem.set(0xFF0F, new);
}

pub fn fetch_interrupt(mem: &mut Memory) -> Option<Interrupt> {
    let available_enabled = mem.get(0xFF0F) & mem.get(0xFFFF);

    if available_enabled & INT_VBLANK != 0 {
        Some(Interrupt::VBlank)
    } else if available_enabled & INT_LCDSTAT != 0 {
        Some(Interrupt::LcdStat)
    } else if available_enabled & INT_TIMER != 0 {
        Some(Interrupt::Timer)
    } else if available_enabled & INT_JOYPAD != 0 {
        Some(Interrupt::Joypad)
    } else {
        None
    }
}

pub fn reset_interrupt(int: Interrupt, mem: &mut Memory) {
    let current = mem.get(0xFF0F);

    let new = match int {
        Interrupt::VBlank => current & !INT_VBLANK,
        Interrupt::LcdStat => current & !INT_LCDSTAT,
        Interrupt::Timer => current & !INT_TIMER,
        Interrupt::Joypad => current & !INT_JOYPAD,
    };

    mem.set(0xFF0F, new);
}
