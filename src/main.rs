#![no_std]
#![no_main]

use arduino_hal::{entry, Peripherals};
use core::panic::PanicInfo;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(500);
    }
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}
