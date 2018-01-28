use memory::Memory;
use math;

pub enum Button {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
}

pub struct Input {
    col1: u8,
    col2: u8,
    active_col1: bool,
    has_interrupt: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            col1: 0b00010000,
            col2: 0b00100000,
            active_col1: true,
            has_interrupt: false,
        }
    }

    pub fn set_input(&mut self, key: Button, key_down: bool) {
        let func = if key_down { math::reset } else { math::set };

        let mut col1_change = false;

        match key {
            Button::A => {
                self.col2 = func(self.col2, 0);
            }
            Button::B => {
                self.col2 = func(self.col2, 1);
            }
            Button::Select => {
                self.col2 = func(self.col2, 2);
            }
            Button::Start => {
                self.col2 = func(self.col2, 3);
            }
            Button::Right => {
                self.col1 = func(self.col1, 0);
                col1_change = true;
            }
            Button::Left => {
                self.col1 = func(self.col1, 1);
                col1_change = true;
            }
            Button::Up => {
                self.col1 = func(self.col1, 2);
                col1_change = true;
            }
            Button::Down => {
                self.col1 = func(self.col1, 3);
                col1_change = true;
            }
        }

        self.has_interrupt |= self.active_col1 == col1_change;
    }

    pub fn fetch_interrupt(&mut self) -> bool {
        let res = self.has_interrupt;
        self.has_interrupt = false;

        res
    }

    pub fn update(&mut self, mem: &mut Memory) {
        self.active_col1 = (mem.get(0xFF00) & 0x10) != 0;

        mem.set(
            0xFF00,
            0xC0 | if self.active_col1 {
                self.col1
            } else {
                self.col2
            },
        );
    }
}
