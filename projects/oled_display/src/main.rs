use std::thread;
use std::time::Duration;

use esp_idf_hal::{
    gpio::{Level, PinDriver},
    peripherals::Peripherals,
};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut led = PinDriver::output(pins.gpio33).unwrap();

    loop {
        if led.is_set_low() {
            led.set_level(Level::High).expect("Failed to set LED high");
            log::info!("LED is ON");
        } else {
            led.set_level(Level::Low).expect("Failed to set LED low");
            log::info!("LED is OFF");
        }

        // thread::sleep to make sure the watchdog won't trigger
        thread::sleep(Duration::from_millis(500));
    }
}
