// src/main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Conditional imports
#[cfg(feature = "host")]
use embedded_hal_mock::pin::{Mock as PinMock, State as PinState, Transaction as PinTrans};

#[cfg(not(feature = "host"))]
use esp_idf_hal::prelude::*;

#[no_mangle]
extern "C" fn app_main() {
    // Initialize logger (only on ESP32)
    #[cfg(not(feature = "host"))]
    {
        esp_idf_sys::link_patches();
        esp_idf_svc::log::EspLogger::initialize_default();
        log::info!("Hello, ESP32!");
    }

    // Blink logic
    #[cfg(not(feature = "host"))]
    {
        let peripherals = Peripherals::take().unwrap();
        let mut led = peripherals.pins.gpio2.into_output().unwrap();

        loop {
            led.set_high().unwrap();
            esp_idf_hal::delay::FreeRtos::delay_ms(500);
            led.set_low().unwrap();
            esp_idf_hal::delay::FreeRtos::delay_ms(500);
        }
    }

    #[cfg(feature = "host")]
    {
        // Mock expectations: toggle between Low/High
        let expectations = [
            PinTrans::set(PinState::High),
            PinTrans::set(PinState::Low),
        ];
        let mut mock = PinMock::new(&expectations);
        mock.set(PinState::High).unwrap();
        mock.set(PinState::Low).unwrap();
        // Validate
        mock.done();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(not(feature = "host"))]
    log::error!("Panic: {:?}", info);

    loop {}
}