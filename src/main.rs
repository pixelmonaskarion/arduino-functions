#![no_std]
#![no_main]

use arduino_hal::{port::{mode::{PullUp, Input}, Pin}, hal::port::PD2};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let button = pins.d2.into_pull_up_input();

    loop {
        let button_state = read_button_state(&button);
        if button_state {
            led.set_high();
        } else {
            led.set_low();
        }
        arduino_hal::delay_ms(10);
    }
}

fn read_button_state(button: &Pin<Input<PullUp>, PD2>) -> bool {
    button.is_high()
}