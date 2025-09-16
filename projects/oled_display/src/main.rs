use anyhow::anyhow;
use std::thread;
use std::time::Duration;

use esp_idf_hal::{
    i2c::{I2cConfig, I2cDriver},
    gpio::{ PinDriver},
    peripherals::Peripherals,
    prelude::*,
};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let i2c = peripherals.i2c0;
    let sda = pins.gpio21;
    let scl = pins.gpio22;

    let mut led = PinDriver::output(pins.gpio33).unwrap();


    let config = I2cConfig::new().baudrate(400u32.kHz().into());
    let i2c_driver = I2cDriver::new(i2c, sda, scl, &config)
        .map_err(|e| anyhow!("I2C init error: {:?}", e))
        .unwrap();

    let interface = I2CDisplayInterface::new(i2c_driver);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display
        .init()
        .map_err(|e| anyhow!("Display init error: {:?}", e))
        .unwrap();
    display
        .flush()
        .map_err(|e| anyhow!("Flush error: {:?}", e))
        .unwrap();

    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello World !!!", Point::new(10, 32), style)
        .draw(&mut display)
        .map_err(|e| anyhow!("Draw error: {:?}", e))
        .unwrap();

    display
        .flush()
        .map_err(|e| anyhow!("Final flush error: {:?}", e))
        .unwrap();

    loop {
        let _ = led.toggle();
        // thread::sleep to make sure the watchdog won't trigger
        thread::sleep(Duration::from_millis(500));
    }
}
