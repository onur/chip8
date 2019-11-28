#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::unreadable_literal)]

mod display;
mod emulator;
mod instruction;
mod keyboard;

use emulator::Emulator;
use std::env::args;

fn main() {
    Emulator::new()
        .rom_oku(args().nth(1).unwrap_or_else(|| "brix.ch8".to_string()))
        .expect("ROM okurken hata oluştu")
        .emulate();
}
