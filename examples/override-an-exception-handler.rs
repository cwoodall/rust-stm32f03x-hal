//! How to override an exception handler

#![feature(asm)] // for `bkpt!`
#![no_std]

#[macro_use] // for `bkpt!`
extern crate stm32f03x_hal;

use stm32f0xx::asm;

use core::ptr;

use stm32f03x_hal::exceptions::{self, Exceptions};
use stm32f03x_hal::interrupts::{self, Interrupts};

fn main() {
    // Read an invalid memory address. This triggers a "hard fault" exception
    unsafe {
        ptr::read_volatile(0x2FFF_FFFF as *const u32);
    }
}

#[no_mangle]
pub static _EXCEPTIONS: Exceptions = Exceptions {
    // Here we override the default handler with a `custom_handler` but only
    // for hard fault exceptions.
    hard_fault: custom_handler,
    ..exceptions::DEFAULT_HANDLERS
};

unsafe extern "C" fn custom_handler() {
    // Once you hit the exception in `main`, you should reach this point!
    asm::bkpt();
}

#[no_mangle]
pub static _INTERRUPTS: Interrupts = Interrupts { ..interrupts::DEFAULT_HANDLERS };
