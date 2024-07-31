use core::ptr::{read_volatile, write_volatile};

use crate::rcc::RCC;

const GPIO_BLOCK_SIZE: u32 = 0x400;
const GPIO_BASE_ADDR: u32 = 0x48000000;
const GPIO_MODE_OFFSET: u32 = 0;
const GPIO_OTYPE_OFFSET: u32 = 0x4;
const GPIO_SPEED_OFFSET: u32 = 0x8;
const GPIO_PUPDR_OFFSET: u32 = 0xC;
const GPIO_BSRR_OFFSET: u32 = 0x18;

const GPIO_A_BASE_ADDR: u32 = GPIO_BASE_ADDR + (GPIO_BLOCK_SIZE * 0);
const GPIO_B_BASE_ADDR: u32 = GPIO_BASE_ADDR + (GPIO_BLOCK_SIZE * 1);
const GPIO_C_BASE_ADDR: u32 = GPIO_BASE_ADDR + (GPIO_BLOCK_SIZE * 2);
const GPIO_D_BASE_ADDR: u32 = GPIO_BASE_ADDR + (GPIO_BLOCK_SIZE * 3);
const GPIO_E_BASE_ADDR: u32 = GPIO_BASE_ADDR + (GPIO_BLOCK_SIZE * 4);
const GPIO_F_BASE_ADDR: u32 = GPIO_BASE_ADDR + (GPIO_BLOCK_SIZE * 5);

pub const GPIO_A_SHIFT_AMOUNT: u8 = 17;
pub const GPIO_B_SHIFT_AMOUNT: u8 = 18;
pub const GPIO_C_SHIFT_AMOUNT: u8 = 19;
pub const GPIO_D_SHIFT_AMOUNT: u8 = 20;
pub const GPIO_E_SHIFT_AMOUNT: u8 = 21;
pub const GPIO_F_SHIFT_AMOUNT: u8 = 22;

pub enum GPIOBank {
    GPIOA,
    GPIOB,
    GPIOC,
    GPIOD,
    GPIOE,
    GPIOF,
}

pub enum GPIOMode {
    GpioModeInput,
    GpioModeOutput,
    GpioModeAlternate,
    GpioModeAnalog,
}

impl GPIOMode {
    fn value(&self) -> u32 {
        match *self {
            GPIOMode::GpioModeInput => 0b00,
            GPIOMode::GpioModeOutput => 0b01,
            GPIOMode::GpioModeAlternate => 0b10,
            GPIOMode::GpioModeAnalog => 0b11,
        }
    }
}

pub enum GPIOType {
    GpioTypePushPull,
    GpioTypeOpenDrain,
}

impl GPIOType {
    fn value(&self) -> u32 {
        match *self {
            GPIOType::GpioTypePushPull => 0b00,
            GPIOType::GpioTypeOpenDrain => 0b01,
        }
    }
}

pub enum GPIOSpeed {
    GpioSpeedLow,
    GpioSpeedMedium,
    GpioSpeedHigh,
}

impl GPIOSpeed {
    fn value(&self) -> u32 {
        match *self {
            GPIOSpeed::GpioSpeedLow => 0b00,
            GPIOSpeed::GpioSpeedMedium => 0b01,
            GPIOSpeed::GpioSpeedHigh => 0b11,
        }
    }
}

pub enum GPIOResistor {
    GpioPupdrNone,
    GpioPupdrPullUp,
    GpioPupdrPullDown,
}

impl GPIOResistor {
    fn value(&self) -> u32 {
        match *self {
            GPIOResistor::GpioPupdrNone => 0b00,
            GPIOResistor::GpioPupdrPullUp => 0b01,
            GPIOResistor::GpioPupdrPullDown => 0b10,
        }
    }
}

pub struct GPIO {
    gpio_port_addr: u32,
    gpio_pin_num: u32,
    gpio_mode: u32,
    gpio_type: u8,
    gpio_speed: u8,
    gpio_pupdr: u8,
}

impl GPIO {
    pub fn init_gpio(gpio_bank: GPIOBank, pin_num: &u32) -> GPIO {
        let rcc: RCC = RCC::init_rcc();
        let gpio_port_addr: u32;

        match gpio_bank {
            GPIOBank::GPIOA => {
                gpio_port_addr = GPIO_A_BASE_ADDR;
                rcc.init_rcc_for_gpio_bank(GPIO_A_SHIFT_AMOUNT);
            }
            GPIOBank::GPIOB => {
                gpio_port_addr = GPIO_B_BASE_ADDR;
                rcc.init_rcc_for_gpio_bank(GPIO_B_SHIFT_AMOUNT);
            }
            GPIOBank::GPIOC => {
                gpio_port_addr = GPIO_C_BASE_ADDR;
                rcc.init_rcc_for_gpio_bank(GPIO_C_SHIFT_AMOUNT);
            }
            GPIOBank::GPIOD => {
                gpio_port_addr = GPIO_D_BASE_ADDR;
                rcc.init_rcc_for_gpio_bank(GPIO_D_SHIFT_AMOUNT);
            }
            GPIOBank::GPIOE => {
                gpio_port_addr = GPIO_E_BASE_ADDR;
                rcc.init_rcc_for_gpio_bank(GPIO_E_SHIFT_AMOUNT);
            }
            GPIOBank::GPIOF => {
                gpio_port_addr = GPIO_F_BASE_ADDR;
                rcc.init_rcc_for_gpio_bank(GPIO_F_SHIFT_AMOUNT);
            }
        };

        GPIO {
            gpio_port_addr,
            gpio_pin_num: *pin_num,
            gpio_mode: 0,
            gpio_type: 0,
            gpio_speed: 0,
            gpio_pupdr: 0,
        }
    }

    pub fn configure_gpio_pin(
        &self,
        gpio_mode: GPIOMode,
        gpio_type: GPIOType,
        gpio_speed: GPIOSpeed,
        gpio_resistor: GPIOResistor,
        pin_to_configure: &u32,
    ) {
        let gpio_mode_reg_addr: u32 = self.gpio_port_addr + GPIO_MODE_OFFSET;
        let gpio_type_reg_addr: u32 = self.gpio_port_addr + GPIO_OTYPE_OFFSET;
        let gpio_speed_reg_addr: u32 = self.gpio_port_addr + GPIO_SPEED_OFFSET;
        let gpio_pupdr_reg_addr: u32 = self.gpio_port_addr + GPIO_PUPDR_OFFSET;

        let current_gpio_type_reg_val: u32 =
            unsafe { read_volatile(gpio_type_reg_addr as *const u32) };

        let current_gpio_speed_reg_val: u32 =
            unsafe { read_volatile(gpio_speed_reg_addr as *const u32) };

        let current_gpio_pupdr_reg_val: u32 =
            unsafe { read_volatile(gpio_pupdr_reg_addr as *const u32) };

        match gpio_mode {
            GPIOMode::GpioModeInput => {
                set_gpio_mode_input(gpio_mode_reg_addr as *mut u32, pin_to_configure)
            }
            GPIOMode::GpioModeOutput => {
                set_gpio_mode_output(gpio_mode_reg_addr as *mut u32, pin_to_configure)
            }
            GPIOMode::GpioModeAlternate => {
                set_gpio_mode_alternate(gpio_mode_reg_addr as *mut u32, pin_to_configure)
            },
            GPIOMode::GpioModeAnalog => {
                set_gpio_mode_analog(gpio_mode_reg_addr as *mut u32, pin_to_configure)
            },
        };


    }
}

fn set_gpio_mode_input(gpio_mode_reg_addr: *mut u32, pin_to_configure: &u32) {
    unsafe {
        write_volatile(
            gpio_mode_reg_addr,
            read_volatile(gpio_mode_reg_addr as *const u32)
                & !(!GPIOMode::GpioModeInput.value() << (*pin_to_configure * 2)),
        )
    }
}

fn set_gpio_mode_output(gpio_mode_reg_addr: *mut u32, pin_to_configure: &u32) {
    unsafe {
        write_volatile(
            gpio_mode_reg_addr as *mut u32,
            read_volatile(gpio_mode_reg_addr as *const u32)
                & !(GPIOMode::GpioModeOutput.value() << (*pin_to_configure as u32 * 2 + 1)),
        );
        write_volatile(
            gpio_mode_reg_addr as *mut u32,
            read_volatile(gpio_mode_reg_addr as *const u32)
                | (GPIOMode::GpioModeOutput.value() << (*pin_to_configure as u32 * 2)),
        )
    }
}

fn set_gpio_mode_alternate(gpio_mode_reg_addr: *mut u32, pin_to_configure: &u32) {
    unsafe {
        write_volatile(
            gpio_mode_reg_addr as *mut u32,
            read_volatile(gpio_mode_reg_addr as *const u32)
                & !(GPIOMode::GpioModeAlternate.value() << (*pin_to_configure as u32 * 2)),
        );
        write_volatile(
            gpio_mode_reg_addr as *mut u32,
            read_volatile(gpio_mode_reg_addr as *const u32)
                | (GPIOMode::GpioModeAlternate.value() << (*pin_to_configure as u32 * 2 + 1)),
        )
    }
}

fn set_gpio_mode_analog(gpio_mode_reg_addr: *mut u32, pin_to_configure: &u32) {
    unsafe {
        write_volatile(
            gpio_mode_reg_addr,
            read_volatile(gpio_mode_reg_addr as *const u32)
                | (GPIOMode::GpioModeAnalog.value() << (*pin_to_configure * 2)),
        )
    }
}
