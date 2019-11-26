use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const FOREGROUND_COLOR: u32 = 0x5294e2;
const BACKGROUND_COLOR: u32 = 0x282c34;

pub struct Display {
    buffer: [[u8; WIDTH]; HEIGHT],
    window: Window,
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: [[0; WIDTH]; HEIGHT],
            window: Window::new(
                "Rust ile CHIP-8",
                WIDTH,
                HEIGHT,
                WindowOptions {
                    scale: Scale::X16,
                    ..WindowOptions::default()
                },
            )
            .expect("Pencere oluşturulurken hata oluştu"),
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn clear(&mut self) {
        self.buffer = [[0; WIDTH]; HEIGHT];
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0;
        let mut xi: usize;
        let mut yj: usize;

        for (j, sprite) in sprite.iter().enumerate() {
            for i in 0..8 {
                xi = (x + i) % WIDTH;
                yj = (y + j) % HEIGHT;

                if sprite & (0x80 >> i) != 0 {
                    if self.buffer[yj][xi] == 1 {
                        collision = 1
                    }
                    self.buffer[yj][xi] ^= 1;
                }
            }
        }

        self.draw_screen();
        collision
    }

    pub fn draw_screen(&mut self) {
        let mut buffer = [0; WIDTH * HEIGHT];
        let mut loc = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                buffer[loc] = if self.buffer[y][x] == 1 {
                    FOREGROUND_COLOR
                } else {
                    BACKGROUND_COLOR
                };
                loc += 1;
            }
        }
        self.window.update_with_buffer(&buffer).unwrap();
    }
}

impl AsMut<Window> for Display {
    fn as_mut(&mut self) -> &mut Window {
        &mut self.window
    }
}
