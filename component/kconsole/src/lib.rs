#![no_std]

use core::fmt;
use core::fmt::Write;

mod color;
use color::Color;

extern crate x86;
use x86::shared::io::outb;

const ROWS: usize = 25;
const COLS: usize = 80;

pub struct Console<T: AsMut<[u16]>> {
    slice: T,
    buffer: [u16; ROWS * COLS],
    row: u8,
    col: u8,
}

impl<T: AsMut<[u16]>> Console<T> {
    pub fn new(mut slice: T) -> Console<T> {
        // we must have enough bytes of backing storage to make this work.
        assert_eq!(slice.as_mut().len(), ROWS * COLS);

        Console {
            slice: slice,
            buffer: [0u16; ROWS * COLS],
            row: 0,
            col: 0,
        }
    }

    pub fn flush(&mut self) {
        self.slice.as_mut().clone_from_slice(&self.buffer);
    }

    fn write_byte(&mut self, byte: u8) {
        match byte as char {
            '\n' => {
                self.col = 0;
                self.row = self.row + 1;
            }
            _ => {
                let i = (self.row as usize * 80 + self.col as usize) as usize;
                self.buffer[i] = ((color::make(Color::Gray, Color::Black) as u16) << 8) |
                                 (byte as u16);
                self.col = self.col + 1;
            }
        }

        if self.col >= 80 {
            self.row = self.row + 1;
            self.col = 0;
        }

        self.scroll();

        self.set_cursor();
    }

    fn scroll(&mut self) {
        if self.row >= 25 {
            for i in 0..ROWS {
                self.buffer[i * 80] = self.buffer[(i + 1) * 80];
            }
            self.row = 24;
        }
    }

    fn set_cursor(&self) {
        let cursor_location = (self.row * 80) as u16 + self.col as u16;
        unsafe {
            outb(0x3D4, 14);
            outb(0x3D5, (cursor_location >> 8) as u8);
            outb(0x3D4, 15);
            outb(0x3D5, (cursor_location) as u8);
        }
    }
}

impl<T: AsMut<[u16]>> Write for Console<T> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for b in s.bytes() {
            self.write_byte(b);
        }
        self.flush();
        Ok(())
    }
}
