use minifb::Key;

pub struct Keyboard(Option<u8>);

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard(None)
    }
    pub fn press_key(&mut self, key: Key) {
        self.0 = self.to_chip8_key(key);
    }
    pub fn release_key(&mut self) {
        self.0 = None;
    }
    pub fn pressed_key(&self) -> Option<u8> {
        self.0
    }
    fn to_chip8_key(&self, key: Key) -> Option<u8> {
        match key {
            Key::Key1 => Some(0x01),
            Key::Key2 => Some(0x02),
            Key::Key3 => Some(0x03),
            Key::Key4 => Some(0x0C),

            Key::Q => Some(0x04),
            Key::W => Some(0x05),
            Key::E => Some(0x06),
            Key::R => Some(0x0D),

            Key::A => Some(0x07),
            Key::S => Some(0x08),
            Key::D => Some(0x09),
            Key::F => Some(0x0E),

            Key::Z => Some(0x0A),
            Key::X => Some(0x00),
            Key::C => Some(0x0B),
            Key::V => Some(0x0F),

            _ => None,
        }
    }
}
