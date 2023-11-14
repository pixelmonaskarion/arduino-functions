#![no_std]
#![no_main]

use arduino_hal::{port::{mode::Output, Pin}, hal::port::PB5};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();

    loop {
        blink(&mut led);
    }
}

fn blink(led: &mut Pin<Output, PB5>) {
    led.set_high();
    arduino_hal::delay_ms(1000);
    led.set_low();
    arduino_hal::delay_ms(1000);
}