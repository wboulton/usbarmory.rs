//! Turns the blue LED on and the white LED off

#![no_main]
#![no_std]

use core::time::Duration;

use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{
    println,
    rng::{self, Rng},
    serial::Serial,
    time::Instant,
    led::Leds,
    time,
};


// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtic::app]`
#[no_mangle]
fn main() -> ! {
    let leds = Leds::take().expect("UNREACHABLE");
    let rng = Rng::initialize().expect("vro");
    rng.wait_for_initial_seed();
    loop {
        leds.blue.on();
        leds.white.off();

        // wait 5 seconds
        time::wait(Duration::from_secs((rng.next_u32() as u64) % 5));
        leds.blue.off();
        leds.white.on();
        time::wait(Duration::from_secs((rng.next_u32() as u64) % 5));
    }

}
