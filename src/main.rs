#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate microbit;
extern crate panic_halt;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}

