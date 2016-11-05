#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    Gray = 0x7,
    DarkGray = 0x8,
    BrightBlue = 0x9,
    BrightGreen = 0xA,
    BrightCyan = 0xB,
    BrightRed = 0xC,
    BrightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

pub fn make(fore: Color, back: Color) -> u8 {
    ((back as u8) << 4) | (fore as u8)
}

#[test]
fn test_size() {
    use core::mem::size_of;
    assert_eq!(size_of::<Color>(), 1);
}

#[test]
fn test_color() {
    assert_eq!(make(Color::Blue, Color::BrightMagenta), 0xD1);
    assert_eq!(make(Color::Yellow, Color::Red), 0x4E);
    assert_eq!(make(Color::DarkGray, Color::White), 0xF8);
    assert_eq!(make(Color::White, Color::Black), 0x0F);
    assert_eq!(make(Color::Gray, Color::Black), 0x07);
}