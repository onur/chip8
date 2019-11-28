#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_lossless)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::non_ascii_literal)]

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
