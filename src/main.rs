#![no_std]
#![no_main]

mod gpio;
mod rcc;

extern crate disco_runtime;
use disco_runtime::entry;
use gpio::{GPIOBank, GPIOMode, GPIOResistor, GPIOSpeed, GPIOType, GPIO};

entry!(main);

pub fn main() -> ! {
    const PIN_NUM: u32 = 8;
    let gpio: GPIO = GPIO::init_gpio(GPIOBank::GPIOE, &PIN_NUM);
    gpio.configure_gpio_pin(
        GPIOMode::GpioModeOutput,
        GPIOType::GpioTypePushPull,
        GPIOSpeed::GpioSpeedMedium,
        GPIOResistor::GpioPupdrNone,
        &PIN_NUM,
    );

    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}
