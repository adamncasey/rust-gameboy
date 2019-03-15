#[derive(Debug)]
pub struct Timer {
    /* Seen cpu cycles modulo 16 "Remainder of a base clock seen" */
    remainder_cycles: u8,
    /* Seen baseclock values modulo 64 */
    baseclock_count: u8,

    divider: u8,
    counter: u8,
    modulo: u8,

    control: u8,
    speed: u8,
    enabled: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            remainder_cycles: 0,
            baseclock_count: 0,
            divider: 0,
            counter: 0,
            modulo: 0,
            control: 0,
            speed: 0,
            enabled: false,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => self.divider,
            0xFF05 => self.counter,
            0xFF06 => self.modulo,
            0xFF07 => self.control,
            _ => panic!("read at unsupported timer address 0x{:x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF04 => {
                self.divider = 0;
            }
            0xFF05 => {
                self.counter = val;
            }
            0xFF06 => {
                self.modulo = val;
            }
            0xFF07 => {
                self.control = val;
                self.speed = val & 0b11;
                self.enabled = (val & 0b001) != 0;

                if self.enabled {
                    dbg!("Enabled timer");
                    dbg!(self);
                }
            }
            _ => panic!("read at unsupported timer address 0x{:x}", addr),
        }
    }

    /*
     * Increment timers. Return true on timer interrupt
     */
    pub fn tick(&mut self, cycles: u8) -> bool {
        /*
        * cycles: raw clock cycles, always increments of 4
        * cycles / 4: "M-clock" - memory clock, the 'effective' Hz of the gameboy CPU (1MHz)
        * cycles / 16: "base clock" - the rate of the 1x timer speed ()

        * divider always increments at 16384Hz
        * counter increments according to the value of `speed`:
        *   |----|----------------|------------------|
        *   | 0b | Inc. Frequency | Base Clock Ratio |
        *   |----|----------------|------------------|
        *   | 00 | 4096 Hz        | 64x slower       |
        *   | 01 | 262144 Hz      | 1x               |
        *   | 10 | 65536 Hz       | 4x slower        |
        *   | 11 | 16384 Hz       | 16x slower       |
        *   |----|----------------|------------------|
        */

        if !self.enabled {
            return false;
        }

        self.remainder_cycles += cycles;

        if self.remainder_cycles > 16 {
            self.baseclock_count += 1;
            self.remainder_cycles -= 16;
        }

        if self.baseclock_count % 16 == 0 {
            self.divider.wrapping_add(1);
        }

        // TODO Turn speed into an enum?
        let ratio = match self.speed {
            0b00 => 64,
            0b01 => 1,
            0b10 => 4,
            0b11 => 16,
            _ => panic!("Unknown speed value {}", self.speed),
        };

        let mut interrupt = false;
        if self.baseclock_count % ratio == 0 {
            let old_count = self.counter;
            self.counter.wrapping_add(1);

            if old_count > self.counter {
                self.counter = self.modulo;
                interrupt = true
            }
        }

        if self.baseclock_count > 64 {
            self.baseclock_count -= 64;
        }

        interrupt
    }
}
