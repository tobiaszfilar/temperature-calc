#![no_std]
#![no_main]

use bme280::i2c::BME280;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::Io, i2c::I2C, peripherals::Peripherals, prelude::*,
    system::SystemControl,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let sda = io.pins.gpio4;
    let scl = io.pins.gpio5;

    let i2c = I2C::new(peripherals.I2C0, sda, scl, 400.kHz(), &clocks, None);

    let mut bme280 = BME280::new_primary(i2c);
    bme280.init(&mut delay).unwrap();

    esp_println::logger::init_logger_from_env();

    loop {
        let measurements = bme280.measure(&mut delay).unwrap();

        log::info!("Relative Humidity = {}%", measurements.humidity);
        log::info!("Temperature = {} deg C", measurements.temperature);
        log::info!("Pressure = {} pascals", measurements.pressure);
        delay.delay(500.millis());
    }
}
