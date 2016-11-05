#![feature(lang_items)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]

#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate x86;
extern crate rlibc;

extern crate kconsole;

mod kcore;
use kcore::KCore;

use core::fmt::Write;

lazy_static!{
    static ref KCORE: KCore = KCore::new();
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    KCORE.console.lock().write_str("hello world\n");
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn rust_begin_panic() -> ! {
    loop {}
}