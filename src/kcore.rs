use core::slice;

extern crate kconsole;
use kconsole::Console;

use spin::Mutex;

pub struct KCore {
    pub console: Mutex<Console<&'static mut [u16]>>,
}

impl KCore {
    pub fn new() -> KCore {
        let _slice = unsafe { slice::from_raw_parts_mut(0xb8000 as *mut u16, 2000) };
        KCore { console: Mutex::new(Console::new(_slice)) }
    }
}