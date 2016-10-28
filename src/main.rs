#![feature(lang_items)]
#![feature(asm)]
#![feature(naked_functions)]
// #![feature(core_intrinsics)]

#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn rust_begin_panic() -> ! {
    loop {}
}