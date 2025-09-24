use std::thread;
use std::time::Duration;

use esp_idf_hal::{
    gpio::{Level, PinDriver, Pull},
    peripherals::Peripherals,
};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut red_light = PinDriver::output(pins.gpio25).unwrap();
    let mut green_light = PinDriver::output(pins.gpio33).unwrap();
    let mut yellow_light = PinDriver::output(pins.gpio32).unwrap();

    let mut button_pin = PinDriver::input(pins.gpio5).unwrap();
    button_pin.set_pull(Pull::Up).unwrap();

    let mut is_pressed = false;
    let mut frames = 0;
    let frame_length = 50;

    loop {
        if button_pin.is_low() && !is_pressed {
            log::info!("Button Pressed!");
            is_pressed = true;
        } else if button_pin.is_high() && is_pressed {
            log::info!("Button Released!");
            is_pressed = false;
        }

        frames += 1;
        if frames >= frame_length {
            frames = 0;
        }

        if red_light.is_set_high() && frames == 0 {
            red_light
                .set_level(Level::Low)
                .expect("Failed to set RED low");
            yellow_light
                .set_level(Level::Low)
                .expect("Failed to set YELLOW low");
            green_light
                .set_level(Level::High)
                .expect("Failed to set GREEN high");
            // log::info!("Green Light!");
        } else if green_light.is_set_high() && frames == 0 {
            red_light
                .set_level(Level::Low)
                .expect("Failed to set RED low");
            yellow_light
                .set_level(Level::High)
                .expect("Failed to set YELLOW high");
            green_light
                .set_level(Level::Low)
                .expect("Failed to set GREEN high");
            // log::info!("Yellow Light!");
        } else if frames == 0 {
            red_light
                .set_level(Level::High)
                .expect("Failed to set RED high");
            yellow_light
                .set_level(Level::Low)
                .expect("Failed to set YELLOW low");
            green_light
                .set_level(Level::Low)
                .expect("Failed to set GREEN low");
            // log::info!("Red Light!");
        }

        // thread::sleep to make sure the watchdog won't trigger
        thread::sleep(Duration::from_millis(10));
    }
}
