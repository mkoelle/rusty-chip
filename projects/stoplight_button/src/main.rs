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

    let mut advance_button = PinDriver::input(pins.gpio5).unwrap();
    advance_button.set_pull(Pull::Up).unwrap();
    let mut auto_button = PinDriver::input(pins.gpio18).unwrap();
    auto_button.set_pull(Pull::Up).unwrap();

    let mut is_advance_pressed = false;
    let mut is_auto_pressed = false;
    let mut frames = 0;
    let frame_length = 50;
    let mut auto_mode = true; // Start in automatic mode
    let mut current_light_state = 0; // 0: Red, 1: Green, 2: Yellow

    loop {
        // Handle advance button (manual step through lights)
        if advance_button.is_low() && !is_advance_pressed {
            log::info!("Advance Button Pressed!");
            is_advance_pressed = true;

            if !auto_mode {
                // Only advance manually when not in auto mode
                current_light_state = (current_light_state + 1) % 3;
                log::info!(
                    "Manual advance to: {}",
                    match current_light_state {
                        0 => "Red",
                        1 => "Green",
                        2 => "Yellow",
                        _ => "Unknown",
                    }
                );
            }
        } else if advance_button.is_high() && is_advance_pressed {
            log::info!("Advance Button Released!");
            is_advance_pressed = false;
        }

        // Handle auto button (toggle automatic cycling)
        if auto_button.is_low() && !is_auto_pressed {
            log::info!("Auto Button Pressed!");
            is_auto_pressed = true;
            auto_mode = !auto_mode;
            log::info!("Auto mode: {}", if auto_mode { "ON" } else { "OFF" });

            // Reset frame counter when switching modes
            frames = 0;
        } else if auto_button.is_high() && is_auto_pressed {
            log::info!("Auto Button Released!");
            is_auto_pressed = false;
        }

        // Handle automatic cycling
        if auto_mode {
            frames += 1;
            if frames >= frame_length {
                frames = 0;
                current_light_state = (current_light_state + 1) % 3;
            }
        }

        // Set light states based on current_light_state
        match current_light_state {
            0 => {
                // Red light
                red_light
                    .set_level(Level::High)
                    .expect("Failed to set RED high");
                yellow_light
                    .set_level(Level::Low)
                    .expect("Failed to set YELLOW low");
                green_light
                    .set_level(Level::Low)
                    .expect("Failed to set GREEN low");
            }
            1 => {
                // Green light
                red_light
                    .set_level(Level::Low)
                    .expect("Failed to set RED low");
                yellow_light
                    .set_level(Level::Low)
                    .expect("Failed to set YELLOW low");
                green_light
                    .set_level(Level::High)
                    .expect("Failed to set GREEN high");
            }
            2 => {
                // Yellow light
                red_light
                    .set_level(Level::Low)
                    .expect("Failed to set RED low");
                yellow_light
                    .set_level(Level::High)
                    .expect("Failed to set YELLOW high");
                green_light
                    .set_level(Level::Low)
                    .expect("Failed to set GREEN low");
            }
            _ => {} // Should never happen
        }

        // thread::sleep to make sure the watchdog won't trigger
        thread::sleep(Duration::from_millis(10));
    }
}
