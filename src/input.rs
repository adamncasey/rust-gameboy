use crate::math;

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
    buttons: u8,
    joypad: u8,
    high4: u8,
}

impl Input {
    pub fn new() -> Input {
        Input {
            buttons: 0,
            joypad: 0,
            high4: 0xF0,
        }
    }

    pub fn set_input(&mut self, key: Button, key_down: bool) {
        let func = if key_down { math::set } else { math::reset };

        match key {
            Button::A => {
                self.buttons = func(self.buttons, 0);
            }
            Button::B => {
                self.buttons = func(self.buttons, 1);
            }
            Button::Select => {
                self.buttons = func(self.buttons, 2);
            }
            Button::Start => {
                self.buttons = func(self.buttons, 3);
            }
            Button::Right => {
                self.joypad = func(self.joypad, 0);
            }
            Button::Left => {
                self.joypad = func(self.joypad, 1);
            }
            Button::Up => {
                self.joypad = func(self.joypad, 2);
            }
            Button::Down => {
                self.joypad = func(self.joypad, 3);
            }
        }

        // TODO interrupt
    }

    pub fn value(&self) -> u8 {
        let mut result = self.high4;
        if (self.high4 & 0x10) != 0 {
            result |= !self.buttons;
        } else if (self.high4 & 0x20) != 0 {
            result |= !self.joypad;
        }

        result
    }

    pub fn update(&mut self, val: u8) {
        self.high4 = val & 0xF0;
    }
}
