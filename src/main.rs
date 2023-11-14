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
        led_on(&mut led);
    led_off(&mut led);
    }
}

fn led_off(led: &mut Pin<Output, PB5>) {
    led.set_low();
    arduino_hal::delay_ms(1000);
}

fn led_on(led: &mut Pin<Output, PB5>) {
    led.set_high();
    arduino_hal::delay_ms(1000);
}