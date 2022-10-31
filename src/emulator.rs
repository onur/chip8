use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use crate::display::Display;
use crate::instruction::Instruction;
use crate::keyboard::Keyboard;

pub struct Emulator {
    /// Bellek 4KB
    memory: [u8; 4096],

    /// Genel amaçlı registerlar
    v: [u8; 16],

    /// Bellek adresini tutmaya yarayan register
    i: u16,

    /// Program sayacı (program counter)
    pc: u16,

    /// Stack
    stack: [u16; 16],

    /// Stack'ın en üstünü tutan stack pointeri
    sp: u8,

    /// Gecikme zamanlayıcısı
    delay_timer: u8,

    /// Ses zamanlayıcısı
    sound_timer: u8,

    /// Görüntü yapısı
    display: Display,

    /// Klavye yapısı
    keyboard: Keyboard,
}

impl Emulator {
    /// Yeni bir Emulator instance'ı yaratır
    pub fn new() -> Emulator {
        let mut emulator = Emulator {
            memory: [0; 4096],
            v: [0; 16],
            i: 0x200,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: Display::new(),
            keyboard: Keyboard::new(),
        };

        for (i, font) in FONTSET.iter().enumerate() {
            emulator.memory[i] = *font;
        }

        emulator
    }

    pub fn rom_oku<P: AsRef<Path>>(mut self, path: P) -> io::Result<Emulator> {
        let file = File::open(path)?;
        for (loc, byte) in file.bytes().enumerate() {
            self.memory[0x200 + loc] = byte?;
        }
        Ok(self)
    }

    /// Bellek üzerinden bir sonraki OPCODE'u alır ve yeni bir Instrution döner
    fn instruction_oku(&self) -> Option<Instruction> {
        let opcode = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[(self.pc + 1) as usize] as u16);
        Instruction::new(opcode)
    }

    fn instruction_calistir(&mut self, instruction: Instruction) {
        self.pc = match instruction {
            Instruction::ClearDisplay => {
                self.display.clear();
                self.pc + 2
            }
            Instruction::Return => {
                self.sp -= 1;
                self.stack[self.sp as usize] + 2
            }
            Instruction::Jump(addr) => addr,
            Instruction::Call(addr) => {
                self.stack[self.sp as usize] = self.pc as u16;
                self.sp += 1;
                addr
            }
            Instruction::SkipIfEqualsByte(x, v) => {
                if self.v[x] == v {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            }
            Instruction::SkipIfNotEqualsByte(x, v) => {
                if self.v[x] != v {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            }
            Instruction::SkipIfEqual(x, y) => {
                if self.v[x] == self.v[y] {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            }
            Instruction::LoadByte(x, v) => {
                self.v[x] = v;
                self.pc + 2
            }
            Instruction::AddByte(x, v) => {
                let (res, _) = self.v[x].overflowing_add(v);
                self.v[x] = res;
                self.pc + 2
            }
            Instruction::Move(x, y) => {
                self.v[x] = self.v[y];
                self.pc + 2
            }
            Instruction::Or(x, y) => {
                self.v[x] |= self.v[y];
                self.pc + 2
            }
            Instruction::And(x, y) => {
                self.v[x] &= self.v[y];
                self.pc + 2
            }
            Instruction::Xor(x, y) => {
                self.v[x] ^= self.v[y];
                self.pc + 2
            }
            Instruction::Add(x, y) => {
                let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = res;
                self.v[0x0F] = if overflow { 1 } else { 0 };
                self.pc + 2
            }
            Instruction::Sub(x, y) => {
                let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = res;
                self.v[0x0F] = if overflow { 0 } else { 1 };
                self.pc + 2
            }
            Instruction::ShiftRight(x) => {
                self.v[0x0F] = self.v[x] & 0x1;
                self.v[x] >>= 1;
                self.pc + 2
            }
            Instruction::ReverseSub(x, y) => {
                self.v[0x0F] = if self.v[x] > self.v[y] { 0 } else { 1 };
                self.v[x] = self.v[y] - self.v[x];
                self.pc + 2
            }
            Instruction::ShiftLeft(x) => {
                self.v[0x0F] = self.v[x] >> 7;
                self.v[x] <<= 1;
                self.pc + 2
            }
            Instruction::SkipIfNotEqual(x, y) => {
                if self.v[x] != self.v[y] {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            }
            Instruction::LoadI(addr) => {
                self.i = addr;
                self.pc + 2
            }
            Instruction::JumpPlusZero(addr) => addr + (self.v[0] as u16),
            Instruction::Random(x, val) => {
                self.v[x] = val & rand::random::<u8>();
                self.pc + 2
            }
            Instruction::Draw(x, y, n) => {
                let from = self.i as usize;
                let to = from + n as usize;
                let x = self.v[x];
                let y = self.v[y];
                self.v[0x0F] = self
                    .display
                    .draw(x as usize, y as usize, &self.memory[from..to]);
                self.pc + 2
            }
            Instruction::SkipIfPressed(x) => {
                if self
                    .keyboard
                    .pressed_key()
                    .map_or(false, |key| key == self.v[x])
                {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            }
            Instruction::SkipIfNotPressed(x) => {
                if self
                    .keyboard
                    .pressed_key()
                    .map_or(false, |key| key == self.v[x])
                {
                    self.pc + 2
                } else {
                    self.pc + 4
                }
            }
            Instruction::LoadDelayTimer(x) => {
                self.v[x] = self.delay_timer;
                self.pc + 2
            }
            Instruction::WaitForKeyPress(x) => {
                if let Some(key) = self.keyboard.pressed_key() {
                    self.v[x] = key;
                    self.pc + 2
                } else {
                    self.pc
                }
            }
            Instruction::SetDelayTimer(x) => {
                self.delay_timer = self.v[x];
                self.pc + 2
            }
            Instruction::SetSoundTimer(x) => {
                self.sound_timer = self.v[x];
                self.pc + 2
            }
            Instruction::AddToI(x) => {
                self.i += self.v[x] as u16;
                self.pc + 2
            }
            Instruction::LoadSprite(x) => {
                self.i = self.v[x] as u16 * 5;
                self.pc + 2
            }
            Instruction::BCDRepresentation(x) => {
                self.memory[self.i as usize] = self.v[x] / 100;
                self.memory[self.i as usize + 1] = (self.v[x] / 10) % 10;
                self.memory[self.i as usize + 2] = (self.v[x] % 100) % 10;
                self.pc + 2
            }
            Instruction::StoreRegisters(x) => {
                for i in 0..=x {
                    self.memory[self.i as usize + i] = self.v[i]
                }
                self.i += x as u16 + 1;
                self.pc + 2
            }
            Instruction::LoadRegisters(x) => {
                for i in 0..=x {
                    self.v[i] = self.memory[self.i as usize + i]
                }
                self.i += x as u16 + 1;
                self.pc + 2
            }
        };
    }

    pub fn emulate(&mut self) {
        while self.display.is_open() {
            self.display.as_mut().update();

            if let Some(keys) = self.display.as_mut().get_keys() {
                keys.iter().for_each(|key| self.keyboard.press_key(*key));
                if keys.is_empty() {
                    self.keyboard.release_key();
                }
            }

            let instruction = self.instruction_oku().expect("Bilinmeyen Instruction");
            self.instruction_calistir(instruction);

            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                if self.sound_timer == 1 {
                    println!("BEEP!");
                }
                self.sound_timer -= 1;
            }

            sleep(Duration::from_millis(5));
        }
    }
}

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

#[cfg(test)]
mod tests {
    use super::Emulator;
}
