//! With semihosting you can print message on the host's stdout
//!
//! Check `cortex-m-semihosting`'s documentation to find out how to receive the
//! messages on the host:
//!
//! https://docs.rs/cortex-m-semihosting
//!
//! And make sure you `cargo build` this example with `--features semihosting`!

#![no_std]

#[macro_use]  // for `hprintln!`
extern crate stm32f03x_hal;

use stm32f03x_hal::exceptions::{self, Exceptions};
use stm32f03x_hal::interrupts::{self, Interrupts};

fn main() {
    hprintln!("Hello, world!");
}

#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };
